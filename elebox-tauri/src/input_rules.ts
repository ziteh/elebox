import { ref } from "vue";

export function makeRules(items?: any[]) {
  return ref({
    required: (v: any) => !!v || "Required",
    duplicate: (v: any) =>
      items == undefined ||
      !items.some((i) => i.name === v) ||
      "Already exists",
  });
}
