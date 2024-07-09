const fs = require('fs');
const path = require('path');

const filePath = path.join(__dirname, 'componentizejs.json');

function getPackageRoot(packageName) {
    try {
        const resolvedPath = require.resolve(packageName);
        const packageRoot = path.dirname(resolvedPath);
        return packageRoot;
    } catch (error) {
        console.error(`Could not resolve path for package: ${packageName} :${error}`);
        return null;
    }
}

function readFileAndResolvePaths() {
    if (!fs.existsSync(filePath)) {
        console.error(`File ${filePath} does not exist.`);
        return;
    }

    const fileContent = fs.readFileSync(filePath, 'utf8');
    const jsonArray = JSON.parse(fileContent);

    let out = ""
    jsonArray.forEach(entry => {
        const packageRoot = getPackageRoot(entry.name);
        if (packageRoot) {
            const fullPath = path.resolve(packageRoot, entry.witPath);
            out += `--wit-path ${fullPath} `
        }
    });
    console.log(out)
}

readFileAndResolvePaths();