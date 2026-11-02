import init,{forecast_request} from "./pkg/hunt_planner.js";
await init();
postMessage({type:"ready"});
self.onmessage=({data})=>{const started=performance.now();try{const report=forecast_request(data.text);postMessage({type:"result",id:data.id,report,elapsedMs:performance.now()-started});}catch(error){postMessage({type:"error",id:data.id,message:String(error)});}};
