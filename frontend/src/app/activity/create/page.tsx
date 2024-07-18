"use client";

import { ChangeEvent, ReactElement, useState } from "react";
import { ActivityType } from "@/types/activity";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import { SelectChangeEvent } from "@mui/material/Select";
import SelectWrapper from "@/components/inputs/select";
import ButtonWrapper from "@/components/button";
import { createActivity } from "@/api/activity";
import Stack from "@mui/material/Stack";
import DatePicker from "@/components/inputs/datePicker";
import Page from "@/components/page";
import { useRouter } from "next/navigation";
import { useAccount, useMsal } from "@azure/msal-react";

const activities = [
  ActivityType.Permanence,
  ActivityType.FollowUp,
  ActivityType.Bootstrap,
  ActivityType.Keynote,
  ActivityType.Review,
  ActivityType.Surveillance,
];

const modules = ["Cpe", "Psu", "Mul", "Mat", "Web", "Aia", "None"];

export default function Home() {
  const router = useRouter();
  const { accounts } = useMsal();
  const user = useAccount(accounts[0] || {});

  let [name, setName] = useState<string>("");
  let [asteks, setAsteks] = useState<number>(0);
  let [activityTypeIdx, setActivityTypeIdx] = useState<number>(0);
  let [moduleIdx, setModuleIdx] = useState<number>(0);
  let [startDate, setStartDate] = useState<Date>();
  let [endDate, setEndDate] = useState<Date>();

  function handleChangeName(event: ChangeEvent<HTMLInputElement>) {
    setName(event.currentTarget.value);
  }

  function handleChangeAsteks(event: ChangeEvent<HTMLInputElement>) {
    setAsteks(parseInt(event.currentTarget.value));
  }

  function handleChangeActivityType(
    event: SelectChangeEvent<HTMLInputElement>
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
          value={activityTypeIdx}
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
      user,
      name,
      asteks,
      activities[activityTypeIdx],
      modules[moduleIdx],
      startDate,
      endDate
    ).then((response) => {
      router.push(`/activity/${response.data}`);
    });
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
