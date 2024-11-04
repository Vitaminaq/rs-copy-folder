import { copyDirectory } from './index.js'
import path from "node:path";

const resolve = (p) => path.resolve(process.cwd(), p);

console.time("copy");
copyDirectory(resolve("public"), resolve("dist")).then(() => {
    console.timeEnd("copy");
});