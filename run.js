const fs = require('fs');

const wasm = fs.readFileSync('./target/wasm32-unknown-unknown/release/rust-wasm.wasm');
const mod = new WebAssembly.Instance(new WebAssembly.Module(wasm));
const heap = new Uint8Array(mod.exports.memory.buffer);

function allocStr(mem, str) {
  const buf = Buffer.from(str);
  const ptr = mod.exports.alloc(buf.length + 1);
  heap.set(buf, ptr);
  heap[buf.length] = 0; // write null byte
  return ptr;
}

function copyCStr(mem, ptr) {
  let end = ptr;
  while (mem[end] !== 0) {
    end++;
  }

  return Buffer.from(mem.buffer, ptr, end-ptr).toString();
}

const person = {
  firstName: 'foo',
  lastName: 'bar',
};
const personJson = JSON.stringify(person);
const personPtr = allocStr(heap, personJson);
const greetingPtr = mod.exports.hello(personPtr);
mod.exports.dealloc(personPtr);
const greetingJson = copyCStr(heap, greetingPtr);
mod.exports.dealloc(greetingPtr);
const { message } = JSON.parse(greetingJson);
console.log(message);
