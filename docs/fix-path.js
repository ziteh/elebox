const fs = require('fs');
const path = require('path');

const rootDir = path.join(__dirname, 'out/docs');

const oldPath = './_next/';
const newPath = '../_next/';

function replaceInFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');
  content = content.replace(new RegExp(oldPath, 'g'), newPath);
  fs.writeFileSync(filePath, content, 'utf8');
}

function traverseDirectory(dir) {
  const files = fs.readdirSync(dir);

  files.forEach(file => {
    const fullPath = path.join(dir, file);
    const stats = fs.statSync(fullPath);

    if (stats.isDirectory()) {
      traverseDirectory(fullPath);
    } else if (stats.isFile()) {
      replaceInFile(fullPath);
    }
  });
}

traverseDirectory(rootDir);

console.log('Path replacement complete!');
