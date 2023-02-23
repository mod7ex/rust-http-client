import http from 'http'
import fs  from 'fs';
import { dirname, join }  from 'path';
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
    res.statusCode = 200;

    if(req.url === '/file') {
        res.setHeader('Content-Type', 'application/octet-stream');

        if(req.method === 'HEAD') {
            res.setHeader('Content-Disposition', 'attachment; filename="disposition-header.txt"');
            res.end('');
        }

        if(req.method === 'GET') {
            const filePath = join(__dirname, 'file.txt');
            const stat = fs.statSync(filePath);

            res.setHeader('Content-Length', stat.size);

            const readStream = fs.createReadStream(filePath);
            readStream.pipe(res);
        }
    } else res.end('Some data');

});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});

