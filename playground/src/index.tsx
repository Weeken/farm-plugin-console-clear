import React from 'react';
import { createRoot } from 'react-dom/client';
import { Main } from './main';
import './index.css'


const container = document.querySelector('#root');
const root = createRoot(container);

root.render(<Main />);

let a = 'aaa'
console.log('%c [ a ]-13', 'font-size:13px; background:rgba(66, 184, 131, 0.2); color:#05a15b;', a)
console.log(a)