##### 使用 - nodejs可直接调用
``` bash
npm i @wefly-rs/copy-folder -D
```
``` javascript
import { copyDirectory } from "@wefly-rs/copy-folder";

await copyDirectory(src, dest);
```

##### tip
场景产生于，用来解决rsbuild构建过程中文件拷贝过慢，且同步阻塞问题

##### [napi]
https://github.com/napi-rs/napi-rs/commit/b5a5b032f69c77d85eb52312b9439bff9a8b41d8#diff-6f509a90e9e2bbe086ba3e189167595922d98c8f067b90c533c8c43f6f4445d1R194