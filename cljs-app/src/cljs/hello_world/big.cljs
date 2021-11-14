(ns hello-world.big
  (:require [cljs.loader :as loader]
            [helix.core :refer [$ defnc]]
            [helix.dom :as d]
            [mustache-rs]
            [libhunam :refer [Comp] :rename {Comp LibhunamComp}]))

(defnc app [_]
  (d/div
   (d/p "Big App")
   ($ LibhunamComp {:msg "big"})))

(loader/set-loaded! :big)

