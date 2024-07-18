import { UUID } from "crypto";

export type Activity = {
  id: UUID;
  type: ActivityType;
  start: Date;
  end: Date;
  name: string;
  needed_asteks: number;
  module?: Module;
};

export enum ActivityType {
  FollowUp = "FollowUp",
  Bootstrap = "Bootstrap",
  Review = "Review",
  Keynote = "Keynote",
  Surveillance = "Surveillance",
  Permanence = "Permanence",
}

export enum Module {
  Cpe = "Cpe",
  Psu = "Psu",
  Mul = "Mul",
  Mat = "Mat",
  Web = "Web",
  Aia = "Aia",
}
