import {injectable} from "tsyringe";

interface StadalMemory {
  total: string,
  available: string,
  free: string
}
interface StadalHost {
 name: string,
 release: string,
 version: string,
 hostname: string,
 arch: string,
 uptime: string,
}

@injectable()
export default class Actions {
  display_memory(data: StadalMemory) {
    document.getElementById("mem-total").innerText = data.total;
    document.getElementById("mem-available").innerText = data.available;
    document.getElementById("mem-free").innerText = data.free;
  }
  display_host(data: StadalHost) {
    document.getElementById("host-name").innerText = data.name;
    document.getElementById("host-release").innerText = data.release;
    document.getElementById("host-version").innerText = data.version;
    document.getElementById("host-hostname").innerText = data.hostname;
    document.getElementById("host-arch").innerText = data.arch;
    document.getElementById("host-uptime").innerText = data.uptime;
  }
}
