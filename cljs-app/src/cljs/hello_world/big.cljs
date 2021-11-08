(ns hello-world.big
  (:require [cljs.loader :as loader]
            [libhunam :refer [Comp] :rename {Comp LibhunamComp}]))

(defn app
  []
  [:div
   [:p "Big App"]
   [:> LibhunamComp {:msg "big"}]])

(loader/set-loaded! :big)

