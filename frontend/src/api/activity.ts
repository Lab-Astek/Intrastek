import { Activity, ActivityType, Module } from "@/types/activity";
import { post, get } from "./request";
import { UUID } from "crypto";
import { AxiosResponse } from "axios";
import { AccountInfo } from "@azure/msal-browser";

function moduleFromString(module: string): Module | undefined {
  switch (module) {
    case "Cpe":
      return Module.Cpe;
    case "Psu":
      return Module.Psu;
    case "Mul":
      return Module.Mul;
    case "Mat":
      return Module.Mat;
    case "Web":
      return Module.Web;
    case "Aia":
      return Module.Aia;
    default:
      return undefined;
  }
}

export async function createActivity(
  user: AccountInfo | null,
  name: string,
  asteks: number,
  activityType: ActivityType,
  module: string,
  start: Date,
  end: Date
): Promise<AxiosResponse<UUID, any>> {
  let data: Activity = {
    id: "1-1-1-1-1",
    name: name,
    needed_asteks: asteks,
    type: activityType,
    module: moduleFromString(module),
    start: start,
    end: end,
  };

  return post<UUID>("activities", data, user).then();
}

export async function getActivity(
  user: AccountInfo | null,
  uuid: UUID
): Promise<AxiosResponse<Activity>> {
  return get<Activity>(`activities/${uuid}`, null);
}

export async function getActivities(user: AccountInfo | null) {
  return get<Activity[]>("activities", user);
}
