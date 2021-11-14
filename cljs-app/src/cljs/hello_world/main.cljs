(ns ^:figwheel-load hello-world.main
  (:require
   [cljs.loader :as loader]
   [clojure.core.async :as a :refer [<! go]]
   [clojure.core.async.interop :refer-macros [<p!]]
   [clojure.pprint :as pprint]
   [goog.dom :as gdom]
   [helix.core :refer [$ defnc]]
   [helix.dom :as d]
   [helix.hooks :as hooks]
   [libhunam :refer [fun] :rename {fun libhunam-fun}]
   [mustache-rs]
   [react-dom :as rdom]))

(enable-console-print!)

(defn set-context [ctx] (go (.set-context (<p! mustache-rs) ctx)))

(defn template
  [str]
  (-> mustache-rs
      (.then #(.new (.-Mustache %) str))))

(def mustache-templates
  ["Hello {{ calc.who }}"
   "Hello {{ calc.whoElse }} {{ calc.doesnt.resolve.to.anything }}"])

(def initial-context {})

(def calcs {:who (fn [_] "The Whose??"), :whoElse (fn [_] "Elsie")})

(defnc mustache-test []
  (let [[state setState] (hooks/use-state nil)]
    (when-not state
      (go
       (let [mustaches (<p! (js/Promise.all (map template mustache-templates)))
             deps      (set (mapcat #(.deps % #js ["calc"]) mustaches))
             calcs     (reduce (fn [acc dep]
                                 (let [fun (or (get calcs (keyword dep))
                                               (constantly nil))]
                                   (assoc acc dep (fun initial-context))))
                               {}
                               deps)
             ctx       (clj->js (assoc initial-context :calc calcs))]
         (<! (set-context ctx))
         (setState (map #(.render %) mustaches)))))
    (d/ul
     (doall (map (fn [rendered tpl] (d/li {:key rendered} tpl " => " rendered))
                 state
                 mustache-templates)))))

(defnc app []
  (let [[bigComp setBigComp] (hooks/use-state nil)]
    (d/div {:style {:margin "3em"}}
     (d/p "CLJS + TS + Wasm Hello Worlds")
     ($ mustache-test)
     (d/pre (with-out-str (pprint/pprint {:libhunan/fun (libhunam-fun)})))
     (d/button
      {:on-click (fn [_]
                   (when-not (some? bigComp)
                     (loader/load :big
                                  #(setBigComp (constantly
                                                @(resolve
                                                  'hello-world.big/app))))))}
      (d/span "Load " (d/strong "big") " module"))
     (if-let [comp bigComp]
       ($ comp)
       (d/p {:style {:margin "2em 0 0 2em"}} "placeholder")))))

(rdom/render ($ app) (gdom/getElement "app"))

(loader/set-loaded! :main)
