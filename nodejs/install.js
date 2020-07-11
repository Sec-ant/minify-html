const fs = require('fs');
const https = require('https');
const path = require('path');
const pkg = require('./package.json');

const MAX_DOWNLOAD_ATTEMPTS = 4;

const binaryName = [process.platform, process.arch, process.versions.modules].join('__');
const binaryPath = path.join(__dirname, 'dist', 'native.node');

const wait = ms => new Promise(resolve => setTimeout(resolve, ms));

class StatusError extends Error {
  constructor (status) {
    super(`Bad status of ${status}`);
    this.status = status;
  }
}

const fetch = url => new Promise((resolve, reject) => {
  const stream = https.get(url, resp => {
    if (!resp.statusCode || resp.statusCode < 200 || resp.statusCode > 299) {
      return reject(new StatusError(resp.statusCode));
    }
    const parts = [];
    resp.on('data', chunk => parts.push(chunk));
    resp.on('end', () => resolve(Buffer.concat(parts)));
  });
  stream.on('error', reject);
});

const downloadNativeBinary = async () => {
  for (let attempt = 0; ; attempt++) {
    let binary;
    try {
      binary = await fetch(`https://wilsonl.in/hyperbuild/bin/nodejs/${pkg.version}/${binaryName}.node`);
    } catch (e) {
      if (e instanceof StatusError && attempt < MAX_DOWNLOAD_ATTEMPTS) {
        await wait(Math.random() * 2500 + 500);
        continue;
      }
      throw e;
    }

    fs.writeFileSync(binaryPath, binary);
    break;
  }
};

if (!fs.existsSync(binaryPath)) {
  downloadNativeBinary()
    .then(
      () => console.log(`Downloaded ${pkg.name}`),
      err => {
        console.error(`Failed to download ${pkg.name}: ${err}`);
        process.exit(1);
      },
    );
}
