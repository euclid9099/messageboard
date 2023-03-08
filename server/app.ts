// guide: https://dev.to/am77/deno-v1-0-303j
import { Application } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import Surreal from 'https://deno.land/x/surrealdb@v0.2.0/mod.ts';
import router from './routes.ts';

const HOST = '127.0.0.1';
const PORT = 7700;

const app = new Application();

await Surreal.Instance.connect('http://127.0.0.1:8000/rpc');
await Surreal.Instance.use('global', 'global');

app.use(router.routes());
app.use(router.allowedMethods());

console.log(`Listening on port ${PORT} ...`);
await app.listen(`${HOST}:${PORT}`);