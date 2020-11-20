"use strict"

function load(area, module_path) {
    console.log("Loading", module_path)
    area.className = "loading"
    
    import(module_path)
    .then((module) => {
        module.default().then(function(obj){
            let core = new module.Core(area)
            core.start()
            area.core = core
        }).catch(function(e){
            console.error("Failed to init module:", e)
            area.className = "error"
        })
    }).catch(function(e) {
        console.error("Failed to load:", e)
        area.className = "error"
    });
}

function setup_app() {
    let area = document.getElementById("track_design_app");
    const module_path = './site.js' // Path to WASM JS bindings
    load(area, module_path)
}
window.onload = setup_app



