import { ResponseBuilder } from "@fermyon/spin-sdk";
import { setupExt } from "@fermyon/wasi-ext";

export async function handler(req: Request, res: ResponseBuilder) {
    setupExt()
    console.log(process.env)
    console.log(req);
    res.send("hello universe");
}
