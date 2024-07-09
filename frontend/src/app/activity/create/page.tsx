"use client";

import { ChangeEvent, ReactElement, useState } from "react";
import { ActivitiyType } from "@/types/activity";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import { SelectChangeEvent } from "@mui/material/Select";
import SelectWrapper from "@/components/inputs/select";
import ButtonWrapper from "@/components/button";
import { createActivity, getActivity } from "@/api/activity";
import { Astek } from "@/types/astek";
import Stack from "@mui/material/Stack";
import DatePicker from "@/components/inputs/datePicker";
import Bar from "@/components/bar";
import Page from "@/components/page";

const activities = [
  ActivitiyType.Permanence,
  ActivitiyType.FollowUp,
  ActivitiyType.Bootstrap,
  ActivitiyType.Keynote,
  ActivitiyType.Review,
  ActivitiyType.Surveillance,
];

const modules = ["Cpe", "Psu", "Mul", "Mat", "Web", "Aia", "None"];

export default function Home() {
  let [name, setName] = useState<string>("");
  let [asteks, setAsteks] = useState<number>(0);
  let [activitiyTypeIdx, setActivityTypeIdx] = useState<number>(0);
  let [moduleIdx, setModuleIdx] = useState<number>(0);
  let [startDate, setStartDate] = useState<Date>();
  let [endDate, setEndDate] = useState<Date>();

  let [result, setResult] = useState<Astek>();

  function handleChangeName(event: ChangeEvent<HTMLInputElement>) {
    setName(event.currentTarget.value);
  }

  function handleChangeAsteks(event: ChangeEvent<HTMLInputElement>) {
    setAsteks(parseInt(event.currentTarget.value));
  }

  function handleChangeActivityType(
    event: SelectChangeEvent<HTMLInputElement>,
  ) {
    setActivityTypeIdx(parseInt(event.target.value as string));
  }

  function handleChangeModule(event: SelectChangeEvent<HTMLInputElement>) {
    setModuleIdx(parseInt(event.target.value as string));
  }

  const ActivityPicker = (): ReactElement => {
    return (
      <div className="activity-picker">
        <SelectWrapper
          value={activitiyTypeIdx}
          label="Activity"
          onUpdate={handleChangeActivityType}
        >
          {activities}
        </SelectWrapper>
      </div>
    );
  };

  const ModulePicker = (): ReactElement => {
    return (
      <div className="module-picker">
        <SelectWrapper
          value={moduleIdx}
          label="Module"
          onUpdate={handleChangeModule}
        >
          {modules}
        </SelectWrapper>
      </div>
    );
  };

  function handleSubmit() {
    if (startDate === undefined || endDate === undefined) return;
    createActivity(
      name,
      asteks,
      activities[activitiyTypeIdx],
      modules[moduleIdx],
      { start: startDate, end: endDate },
    );
  }

  return (
    <Page title="Activity creation">
      <form noValidate autoComplete="off">
        <Stack spacing={3}>
          <Box
            component="div"
            sx={{
              "& > :not(style)": { m: 1, width: "25ch" },
            }}
          >
            <TextField
              label="Name"
              margin="dense"
              focused
              onChange={handleChangeName}
            />
            <TextField
              defaultValue={0}
              type="number"
              label="Asteks"
              margin="dense"
              focused
              onChange={handleChangeAsteks}
            />
          </Box>
          <ActivityPicker />
          <ModulePicker />
          <DatePicker
            label="Start"
            onChange={(value, ctx) => {
              if (value) {
                setStartDate(value.toDate());
              }
            }}
          />
          <DatePicker
            label="End"
            onChange={(value, ctx) => {
              if (value) {
                setEndDate(value.toDate());
              }
            }}
          />
          <ButtonWrapper onClick={handleSubmit}>Create</ButtonWrapper>
        </Stack>
      </form>
    </Page>
  );
}
