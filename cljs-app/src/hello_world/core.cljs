(ns hello-world.core
  (:require
   [clojure.core.async :as a :refer [<! go]]
   [reagent.core :as r]
   [clojure.core.async.interop :refer-macros [<p!]]
   [goog.dom :as gdom]
   ["mustache-rs/mustache_rs" :as mustache-rs]
   [reagent.dom :as dom]))

(defn set-context [ctx]
  (go (.set-context (<p! mustache-rs) ctx)))

(defn template
  [str]
  (-> mustache-rs
      (.then #(.new (.-Mustache %) str))))

(def mustache-templates
  ["Hello {{ calc.who }}"
   "Hello {{ calc.whoElse }} {{ calc.doesnt.resolve.to.anything }}"])

(def initial-context
  {})

(def calcs
  {:who     (fn [_] "The Who")
   :whoElse (fn [_] "Elsie")})

(defn mustache-test
  []
  (let [*state (r/atom nil)]
    (go (let [mustaches (<p! (js/Promise.all (map template mustache-templates)))
              deps      (set (mapcat #(.deps % #js ["calc"]) mustaches))
              calcs     (reduce (fn [acc dep]
                                  (let [fun (or (get calcs (keyword dep))
                                                (constantly nil))]
                                (assoc acc dep (fun initial-context))))
                                {} deps)]
          (<! (set-context (clj->js (assoc initial-context :calc calcs))))
          (reset! *state (map #(.render %) mustaches))))
    (fn []
      [:ul
       (doall (map (fn [rendered tpl]
                     [:li {:key rendered
                           :style {:font-size "xxx-large"}}
                      tpl " => " rendered])
                   @*state
                   mustache-templates))])))

(defn simple-component []
  [:div
   [:p "CLJS + Wasm Hello World"]
   [mustache-test]])

(dom/render [simple-component] (gdom/getElement "app"))
