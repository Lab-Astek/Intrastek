import { UUID } from "crypto";
import { post, get } from "./request";
import { Indisponibility } from "@/types/astek";

export async function getAstek(id: UUID) {
  return get(`asteks/${id}`);
}

export async function getAsteks() {
  return get("asteks");
}

export async function createAstek(id: UUID) {
  return post("asteks", id);
}

export async function addIndisponibility(
  id: UUID,
  indisponibility: Indisponibility
) {
  return post(`asteks/${id}`, indisponibility);
}

export async function getIndisponibilities(id: UUID) {
  return get(`asteks/${id}/indisponibilities`);
}

export async function getIndisponibility(id: UUID, indisponibilityId: UUID) {
  return get(`asteks/${id}/indisponibilities/${indisponibilityId}`);
}
