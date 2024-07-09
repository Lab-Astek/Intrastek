import { UUID } from "crypto";
import { post, get } from "./request";

export async function getAstek(id: UUID) {
  return get(`asteks/${id}`);
}

export async function getAsteks() {
  return get("asteks");
}

export async function createAstek(id: UUID) {
  return post("asteks", id);
}

export async function addIndisponibility(id: UUID, indisponibility: any) {
  return post(`asteks/${id}`, indisponibility);
}
