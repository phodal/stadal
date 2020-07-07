import {injectable} from "tsyringe";
import {niceBytes, secondsToHms} from "./format";
import {capitalizeFirstLetter} from "../utils/string-util";

interface StadalMemory {
  total: string,
  available: string,
  free: string,
  swap_total: string,
  swap_free: string,
  swap_used: string,
}

interface StadalHost {
  name: string,
  release: string,
  version: string,
  hostname: string,
  arch: string,
  uptime: string,
}

interface Language {
  name: string,
  version: string,
}

interface CleanSize {
  name: string,
  size: string,
  path: string,
}

interface CPU {
  cores: string,
  current_ghz: string,
  min_ghz: string,
  max_ghz: string,
}

interface Disk {
  device: string,
  filesystem: string,
  mount: string,
  total: string,
  used: string,
  free: string,
}

@injectable()
export default class Actions {
  display_memory(data: StadalMemory) {
    document.getElementById("mem-total").innerText = niceBytes(data.total);
    document.getElementById("mem-available").innerText = niceBytes(data.available);
    document.getElementById("mem-free").innerText = niceBytes(data.free);
    document.getElementById("swap-total").innerText = niceBytes(data.swap_total);
    document.getElementById("swap-free").innerText = niceBytes(data.swap_free);
    document.getElementById("swap-used").innerText = niceBytes(data.swap_used);
  }

  display_host(data: StadalHost) {
    document.getElementById("host-version").innerText = data.version;
    document.getElementById("host-uptime").innerText = secondsToHms(data.uptime);
  }

  display_languages(data: Language[]) {
    let result = "";
    for (let datum of data) {
      result += `${capitalizeFirstLetter(datum.name)} : ${datum.version}<br>`
    }
    document.getElementById("languages").innerHTML = result;
  }

  display_sizes(data: CleanSize[]) {
    let result = "";
    for (let datum of data) {
      result += `${capitalizeFirstLetter(datum.name)} : ${niceBytes(datum.size)} , ${datum.path}<br>`
    }
    document.getElementById("sizes").innerHTML = result;
  }

  display_cpu(data: CPU) {
    let innerHTML =`CPU -> cores:${data.cores}, current: ${data.current_ghz} `;
    document.getElementById("cpu").innerText = innerHTML;
  }

  display_disks(data: Disk[]) {
    let results = '';
    for (let datum of data) {
      let innerHTML =`<div class="memory-content">
<div><span class="title">Device      </span><span class="value"></span>${datum.device}</div>
<div><span class="title">Mount     </span><span class="value">${datum.mount}</span></div>
<div><span class="title">Total     </span><span class="value">${niceBytes(datum.total)}</span></div>
<div><span class="title">Free      </span><span class="value"></span>${niceBytes(datum.free)}</div>
<div><span class="title">Used      </span><span class="value"></span>${niceBytes(datum.used)}</div>
</div>
    `;

      results += innerHTML;
    }
    document.getElementById("disk").innerHTML = results;
  }
}
