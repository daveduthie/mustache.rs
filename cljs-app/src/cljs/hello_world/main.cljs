(ns ^:figwheel-load hello-world.main
  (:require
   [cljs.loader :as loader]
   [clojure.core.async :as a :refer [<! go]]
   [clojure.core.async.interop :refer-macros [<p!]]
   [clojure.pprint :as pprint]
   [goog.dom :as gdom]
   [libhunam :refer [fun] :rename {fun libhunam-fun}]
   [mustache-rs]
   [reagent.core :as r]
   [reagent.dom :as dom]))

(enable-console-print!)

(defn set-context [ctx]
  (go (.set-context (<p! mustache-rs)  ctx)))

(defn template
  [str]
  (-> mustache-rs
      (.then #(.new (.-Mustache %) str))))

(def mustache-templates
  ["Hello {{ calc.who }}"
   "Hello {{ calc.whoElse }} {{ calc.doesnt.resolve.to.anything }}"])

(def initial-context {})

(def calcs {:who     (fn [_] "The Whose??"),
            :whoElse (fn [_] "Elsie")})

(defn mustache-test
  []
  (let [*state (r/atom nil)]
    (go
      (let [mustaches (<p! (js/Promise.all (map template mustache-templates)))
            deps      (set (mapcat #(.deps % #js ["calc"]) mustaches))
            calcs     (reduce (fn [acc dep]
                                (let [fun (or (get calcs (keyword dep))
                                              (constantly nil))]
                                  (assoc acc dep (fun initial-context))))
                              {}
                              deps)]
        (<! (set-context (clj->js (assoc initial-context :calc calcs))))
        (reset! *state (map #(.render %) mustaches))))
    (fn []
      [:ul
       (doall (map (fn [rendered tpl] [:li {:key rendered}
                                       tpl " => " rendered])
                   @*state
                   mustache-templates))])))

(defn app
  []
  (let [*bigComp (r/atom nil)]
    (fn []
      [:div {:style {:margin "3em"}}
       [:p "CLJS + TS + Wasm Hello Worlds"]
       [mustache-test]
       [:pre (with-out-str (pprint/pprint {:libhunan/fun (libhunam-fun)}))]
       [:button
        {:on-click
         (fn [_]
           (when-not (some? @*bigComp)
             (loader/load
              :big
              #(reset! *bigComp (resolve 'hello-world.big/app)))))}
        [:span "Load " [:strong "big"] " module"]]
       (if-let [comp @*bigComp]
         [comp]
         [:p {:style {:margin "2em 0 0 2em"}} "placeholder"])])))

(dom/render [app] (gdom/getElement "app"))

(loader/set-loaded! :main)
