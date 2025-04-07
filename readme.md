# bge-wasm-rust
Rustで構成されたWASM(JavaScript)製BinGameEngineランタイムです。
## これを使用する場合
こちらではバンドラー用・NodeJS用・ブラウザ用を用意しています。
#### バンドラー用
WebAssemblyファイルを`import * as wasm from "bge_wasm_bg.wasm"`で読み込むタイプです。  
バンドラーを使っている人は`bge-wasm/bundler`をインポートしてください。
#### NodeJS用
WebAssemblyファイルをNodeJSの`fs`で読み込むタイプです。  
NodeJS・bun上で実行する人は`bge-wasm`または`bge-wasm/node`をインポートしてください。
#### ブラウザ用
WebAssemblyファイルを`fetch`で読み込むタイプです。  
Web上で実行する人・CDNから使う人・Viteを使う人は`bge-wasm/web`をインポートしてください。
## これを開発する場合のセットアップ
[こちらをご参照ください。](https://qiita.com/osanshouo/items/40f087cc79a1446ad7ef#%E7%92%B0%E5%A2%83%E6%A7%8B%E7%AF%89)
## 状態
だいたいできた、サウンド機能はまたいつか