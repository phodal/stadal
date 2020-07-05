import {injectable} from "tsyringe";

interface StadalMemory {
    total: string,
    available: string,
    free: string
}

@injectable()
export default class Actions {
    display_memory(raw: string) {
        let data: StadalMemory = JSON.parse(raw).params;
        document.getElementById("mem-total").innerText = data.total;
        document.getElementById("mem-available").innerText = data.available;
        document.getElementById("mem-free").innerText = data.free;
    }
}
