import fs  from 'fs';
import { dirname, join }  from 'path';
import { fileURLToPath } from "url";
import express from 'express';
import multer from 'multer';

const upload = multer({ dest: 'uploads/' })

// https://nodejs.bootcss.com/node-request-data

const __dirname = dirname(fileURLToPath(import.meta.url));

const app = express();

app.use(express.urlencoded({ extended: true }));

app.use(express.json());

app.get('/', (_, res) => {
    res.statusCode = 200;
    res.end('Some data');
});

app.head('/file', (_, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/octet-stream');
    res.setHeader('Content-Disposition', 'attachment; filename="disposition-header.txt"');
    res.end('');
});

app.get('/file', (_, res) => {
    res.statusCode = 200;

    res.setHeader('Content-Type', 'application/octet-stream');

    const filePath = join(__dirname, 'file.txt');
    const stat = fs.statSync(filePath);

    res.setHeader('Content-Length', stat.size);

    const readStream = fs.createReadStream(filePath);
    readStream.pipe(res);
});

app.post('/multipart-form-data', upload.single('file'), (req, res) => {
    console.log('------------------------ FILE');
    console.log(req.file);
    console.log('------------------------ BODY');
    console.log(req.body);
    res.end('');
});

app.post('/urlencoded', (req, res) => {
    console.log('------------------------ BODY');
    console.log(req.body);
    console.log('------------------------ BODY');
    res.end('');
});

const HOSTNAME = '127.0.0.1';
const PORT = 3000;

app.listen(() => {

});

app.listen(PORT, () => {
    console.log(`Server running at http://${HOSTNAME}:${PORT}`);
});