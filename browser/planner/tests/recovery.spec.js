import {test,expect} from "@playwright/test";
import fs from "node:fs";
const example=JSON.parse(fs.readFileSync(new URL("../web/example.json",import.meta.url),"utf8"));
const fault=JSON.stringify({kind:"invalid_request",message:"Synthetic lifecycle fault only; not a forecast or golden outcome."});
async function ready(page){await page.goto("./");await expect(page.locator("#run")).toBeEnabled();await expect(page.locator("#input")).not.toHaveValue("");}
async function intercept(page){await page.addInitScript(()=>{const Actual=window.Worker;window.__workers=[];window.__sent=[];window.__hold=false;window.Worker=class{constructor(url,options){this.actual=new Actual(url,options);this.actual.onmessage=e=>this.onmessage?.(e);this.actual.onerror=e=>this.onerror?.(e);window.__workers.push(this);}postMessage(data){window.__sent.push(data);if(!window.__hold)this.actual.postMessage(data);}terminate(){this.terminated=true;this.actual.terminate();}};});}

test("delayed and stuck reads preserve later edits and selections with real read deadline",async({page})=>{
 await page.addInitScript(()=>{const original=File.prototype.arrayBuffer;File.prototype.arrayBuffer=function(){if(this.name==="slow.json")return new Promise(resolve=>setTimeout(()=>original.call(this).then(resolve),300));if(this.name==="stuck.json")return new Promise(()=>{});return original.call(this);};});await ready(page);
 const raw=JSON.stringify(example),file=(name,text)=>({name,mimeType:"application/json",buffer:Buffer.from(text)});
 await page.locator("#file").setInputFiles(file("slow.json",raw));await page.locator("#input").fill("{");await page.waitForTimeout(450);await expect(page.locator("#input")).toHaveValue("{");
 await page.locator("#file").setInputFiles(file("slow.json",raw));await page.locator("#file").setInputFiles(file("new.json",raw+" "));await expect(page.locator("#status")).toContainText("Opened assumptions");await page.waitForTimeout(450);await expect(page.locator("#input")).toHaveValue(raw+" ");
 await page.locator("#file").setInputFiles(file("stuck.json","{}"));await expect(page.locator("#status")).toContainText("File read exceeded 5 seconds",{timeout:6500});await expect(page.locator("#input")).toHaveValue(raw+" ");
});

test("file UTF-8, whole paste, edit and drop bounds preserve meaningful input",async({page})=>{
 await ready(page);const raw=await page.locator("#input").inputValue();
 await page.locator("#file").setInputFiles({name:"large.json",mimeType:"application/json",buffer:Buffer.alloc(65537,32)});await expect(page.locator("#status")).toContainText("file exceeds 64 KiB");await expect(page.locator("#input")).toHaveValue(raw);
 await page.locator("#file").setInputFiles({name:"bad.json",mimeType:"application/json",buffer:Buffer.from([255])});await expect(page.locator("#status")).toContainText("retained");await expect(page.locator("#input")).toHaveValue(raw);
 expect(await page.locator("#input").evaluate(field=>{field.setSelectionRange(0,0);const dt=new DataTransfer();dt.setData("text/plain"," ".repeat(65536));const e=new ClipboardEvent("paste",{clipboardData:dt,bubbles:true,cancelable:true});field.dispatchEvent(e);return e.defaultPrevented;})).toBe(true);await expect(page.locator("#input")).toHaveValue(raw);
 await page.locator("#input").fill(" ".repeat(65537));await page.locator("#run").click();await expect(page.locator("#status")).toContainText("assumptions exceed 64 KiB");await expect(page.locator("#report")).toBeDisabled();
 await page.locator("#input").evaluate((field,text)=>{const dt=new DataTransfer();dt.items.add(new File([text],"dropped.json",{type:"application/json"}));field.dispatchEvent(new DragEvent("drop",{dataTransfer:dt,bubbles:true,cancelable:true}));},raw);await expect(page.locator("#status")).toContainText("Opened assumptions");await expect(page.locator("#input")).toHaveValue(raw);
});

test("edit cancels actual worker and wrong IDs or old generations cannot publish",async({page})=>{
 await intercept(page);await ready(page);await page.evaluate(()=>{window.__hold=true;window.__workers[0].saved=window.__workers[0].onmessage;});await page.locator("#run").click();const edited=JSON.stringify({...example,seed:"new-revision"});await page.locator("#input").fill(edited);expect(await page.evaluate(()=>window.__workers[0].terminated)).toBe(true);await expect(page.locator("#run")).toBeEnabled();
 await page.evaluate(fault=>window.__workers[0].saved({data:{type:"result",id:window.__sent[0].id,report:fault}}),fault);await expect(page.locator("#report")).toBeDisabled();await expect(page.locator("#input")).toHaveValue(edited);
 await page.locator("#run").click();await page.evaluate(fault=>{const sent=window.__sent.at(-1);window.__workers.at(-1).onmessage({data:{type:"result",id:sent.id+100,report:fault}});},fault);await expect(page.locator("#status")).toContainText("Computing Rust");await expect(page.locator("#report")).toBeDisabled();
 await page.evaluate(()=>{window.__hold=false;window.__workers.at(-1).actual.postMessage(window.__sent.at(-1));});await expect(page.locator("#status")).toContainText("Forecast complete");expect(JSON.parse(await page.locator("#raw").textContent()).seed).toBe("new-revision");
});

test("real computation deadline and crash clear exports, fence queued ready, and retain inputs",async({page})=>{
 await intercept(page);await ready(page);const source=await page.locator("#input").inputValue();await page.locator("#run").click();await expect(page.locator("#report")).toBeEnabled();await page.evaluate(()=>{window.__hold=true;const w=window.__workers.at(-1);w.saved=w.onmessage;});await page.locator("#run").click();await expect(page.locator("#status")).toContainText("Forecast exceeded 10 seconds",{timeout:12000});await expect(page.locator("#report")).toBeDisabled();await expect(page.locator("#request")).toBeDisabled();expect(await page.evaluate(()=>window.__workers[0].terminated)).toBe(true);
 await page.evaluate(()=>window.__workers[0].saved({data:{type:"ready"}}));await expect(page.locator("#run")).toBeDisabled();await expect(page.locator("#status")).toContainText("exceeded 10 seconds");
 await page.evaluate(()=>window.__hold=false);await page.locator("#restart").click();await expect(page.locator("#run")).toBeEnabled();await page.locator("#run").click();await expect(page.locator("#report")).toBeEnabled();await page.evaluate(()=>window.__workers.at(-1).onerror({message:"synthetic current worker crash"}));await expect(page.locator("#status")).toContainText("Rust worker failed");await expect(page.locator("#input")).toHaveValue(source);await expect(page.locator("#report")).toBeDisabled();
});

test("real startup deadline retains edits and restarts the current engine",async({page})=>{
 await page.route("**/worker.js",async route=>{await new Promise(resolve=>setTimeout(resolve,13500));try{await route.continue();}catch{}});await page.goto("./");await page.locator("#input").fill("{");await expect(page.locator("#status")).toContainText("startup exceeded 12 seconds",{timeout:15000});await expect(page.locator("#input")).toHaveValue("{");await expect(page.locator("#run")).toBeDisabled();await page.unroute("**/worker.js");await page.locator("#restart").click();await expect(page.locator("#run")).toBeEnabled();await expect(page.locator("#input")).toHaveValue("{");
});

test("admitted maximum and actual WASM memory ceiling remain bounded",async({page})=>{
 await ready(page);const maximum={...example,runs:128,team:{skill:10,parallelism:6},puzzles:Array.from({length:6},()=>({difficulty:10,answer_length:32}))};await page.locator("#input").fill(JSON.stringify(maximum));await page.locator("#run").click();await expect(page.locator("#report")).toBeEnabled();const r=JSON.parse(await page.locator("#raw").textContent());expect(r.candidate.runs).toHaveLength(128);expect(r.baseline.runs).toHaveLength(128);
 const growth=await page.evaluate(async()=>{const m=await import("./pkg/hunt_planner.js"),w=await m.default();const before=w.memory.buffer.byteLength;let refused=false;try{w.memory.grow(1024);}catch(e){refused=e instanceof RangeError;}return {before,after:w.memory.buffer.byteLength,refused};});expect(growth.refused).toBe(true);expect(growth.after).toBe(growth.before);expect(growth.after).toBeLessThanOrEqual(67108864);
});

test("late example cannot replace own input",async({page})=>{
 await page.route("**/example.json",async route=>{await new Promise(resolve=>setTimeout(resolve,350));await route.continue();});await page.goto("./");const own=JSON.stringify({...example,seed:"own-before-example"});await page.locator("#input").fill(own);await page.waitForTimeout(500);await expect(page.locator("#input")).toHaveValue(own);await expect(page.locator("#run")).toBeEnabled();await page.locator("#run").click();await expect(page.locator("#report")).toBeEnabled();await expect(page.locator("#raw")).toContainText("own-before-example");
});

test("malformed current worker reports fail safely with exports cleared",async({page})=>{
 await intercept(page);await ready(page);const source=await page.locator("#input").inputValue();
 for(const report of ["{","null",'{"kind":"report"}']){
  await page.evaluate(()=>window.__hold=true);await page.locator("#run").click();await page.evaluate(report=>{const sent=window.__sent.at(-1);window.__workers.at(-1).onmessage({data:{type:"result",id:sent.id,report,elapsedMs:1}});},report);await expect(page.locator("#status")).toContainText(/unreadable|incomplete/);await expect(page.locator("#report")).toBeDisabled();await expect(page.locator("#request")).toBeDisabled();await expect(page.locator("#input")).toHaveValue(source);await page.evaluate(()=>window.__hold=false);await page.locator("#restart").click();await expect(page.locator("#run")).toBeEnabled();
 }
});
