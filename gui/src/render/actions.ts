import {injectable} from "tsyringe";

@injectable()
export default class Actions {
    display_memory(raw: string) {
        document.getElementById("memory").innerText = raw;
    }
}
