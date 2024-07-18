import { UUID } from "crypto";
import { post, get } from "@/api/request";
import { Astek, Indisponibility } from "@/types/astek";
import { AxiosResponse } from "axios";

export async function getAstek(id: UUID): Promise<AxiosResponse<Astek>> {
  return get<Astek>(`asteks/${id}`);
}

export async function getAsteks(): Promise<AxiosResponse<Astek[]>> {
  return get<Astek[]>("asteks");
}

export async function createAstek(id: UUID): Promise<AxiosResponse<Astek>> {
  return post<Astek>("asteks", id);
}

export async function addIndisponibility(
  id: UUID,
  indisponibility: Indisponibility
): Promise<AxiosResponse<number>> {
  return post<number>(`asteks/${id}`, indisponibility);
}

export async function getIndisponibilities(id: UUID) {
  return get<Indisponibility>(`asteks/${id}/indisponibilities`);
}

export async function getIndisponibility(id: UUID, indisponibilityId: UUID) {
  return get<Indisponibility>(
    `asteks/${id}/indisponibilities/${indisponibilityId}`
  );
}
