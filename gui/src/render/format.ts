const units = ['bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

export function niceBytes(x: string | number) {
  let l = 0;
  let n;
  if (typeof x === "string") {
    n = parseInt(x, 10) || 0;
  } else {
    n = x;
  }

  while (n >= 1024 && ++l) {
    n = n / 1024;
  }

  return (n.toFixed(n < 10 && l > 0 ? 1 : 0) + ' ' + units[l]);
}

export function secondsToHms(d: string) {
  let value = Number(parseInt(d, 10));
  const h = Math.floor(value / 3600);
  const m = Math.floor(value % 3600 / 60);
  const s = Math.floor(value % 3600 % 60);

  const hDisplay = h > 0 ? h + (h == 1 ? " hour, " : " hours, ") : "";
  const mDisplay = m > 0 ? m + (m == 1 ? " minute, " : " minutes, ") : "";
  const sDisplay = s > 0 ? s + (s == 1 ? " second" : " seconds") : "";
  return hDisplay + mDisplay + sDisplay;
}
