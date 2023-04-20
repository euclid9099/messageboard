// guide: https://dev.to/am77/deno-v1-0-303j
import { Application } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import router from "./routes.ts";
import { db_url } from "./helper.ts";

const HOST = "127.0.0.1";
const PORT = 7700;

const app = new Application();

//use only for completly unauthorized requests
await Surreal.Instance.connect(`${db_url}/rpc`);
await Surreal.Instance.use("global", "global");

app.use(oakCors({ origin: "*" }));
app.use(router.routes());
app.use(router.allowedMethods());

console.log(`Listening on port ${PORT} ...`);
await app.listen(`${HOST}:${PORT}`);
