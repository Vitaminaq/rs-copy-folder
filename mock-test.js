const fs = require('fs');

// const filePath = '@wefly-rs/copy-folder.darwin-x64.node';
// fs.stat(filePath, (err, stats) => {
//   if (!err && stats.isFile()) {
//     console.log('文件存在。');
//   } else {
//     console.log('文件不存在。');
//   }
// });

console.log(process.cwd());

const files = fs.readdirSync('.');
files.forEach(file => {
  console.log(file);
});

console.log("test passed !!!");
