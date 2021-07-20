import * as raytracer from "raytracer";

let canvas = document.createElement('canvas')
let container = document.querySelector('#raytracer-container')

container.appendChild(canvas)

raytracer.main(canvas)
