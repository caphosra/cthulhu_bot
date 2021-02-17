import * as http from "http";

//
// A logic to make this bot awake.
//
export function keep_awake() {
    const server = http.createServer((_, res) => {
        res.writeHead(200);
        res.end('ok');
    });

    server.listen(3000);
}
