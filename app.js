const express = require('express')
const app = express()
const port = 3000
const path = require('path');

//app.use(express.static(path.join(__dirname, 'pkg')));

//app.listen(port, () => console.log(`Example app listening at http://localhost:${port}`))

express.static.mime.define({'application/wasm': ['wasm']})

app.use(express.static('./dist', {
  setHeaders: (res) => {
    res.set('Cross-Origin-Opener-Policy', 'same-origin');
    res.set('Cross-Origin-Embedder-Policy', 'require-corp');
  }
}));

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})