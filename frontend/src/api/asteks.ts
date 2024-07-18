import { UUID } from "crypto";
import { post, get } from "@/api/request";
import { Astek, Indisponibility } from "@/types/astek";
import { AxiosResponse } from "axios";
import { AccountInfo } from "@azure/msal-browser";

export async function getAstek(
  user: AccountInfo | null,
  id: UUID
): Promise<AxiosResponse<Astek>> {
  return get<Astek>(`asteks/${id}`, user);
}

export async function getAsteks(
  user: AccountInfo | null
): Promise<AxiosResponse<Astek[]>> {
  return get<Astek[]>("asteks", user);
}

export async function createAstek(
  user: AccountInfo | null,
  id: UUID
): Promise<AxiosResponse<Astek>> {
  return post<Astek>("asteks", id, user);
}

export async function addIndisponibility(
  user: AccountInfo | null,
  id: UUID,
  indisponibility: Indisponibility
): Promise<AxiosResponse<number>> {
  return post<number>(`asteks/${id}`, indisponibility, user);
}

export async function getIndisponibilities(user: AccountInfo, id: UUID) {
  return get<Indisponibility>(`asteks/${id}/indisponibilities`, user);
}

export async function getIndisponibility(
  user: AccountInfo | null,
  id: UUID,
  indisponibilityId: UUID
) {
  return get<Indisponibility>(
    `asteks/${id}/indisponibilities/${indisponibilityId}`,
    user
  );
}
