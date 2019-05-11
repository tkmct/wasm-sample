// defined-in-js.js
export function name() {
  return 'World';
}

export class LocalStorage {
  set(k, v) {
    localStorage.setItem(k, v)
  }

  get(k) {
    return localStorage.getItem(k)
  }
}