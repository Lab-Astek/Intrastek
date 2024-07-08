"use client";

import { ChangeEvent, ReactElement, useState } from 'react'
import { ActivitiyType, Activity } from '@/types/activity';
import TextField from '@mui/material/TextField';
import Box from '@mui/material/Box';
import { SelectChangeEvent } from '@mui/material/Select';
import SelectWrapper from '@/components/inputs/select';
import ButtonWrapper from '@/components/button';
import createActivity from '@/api/activity';

const activities = [
  ActivitiyType.Permanence,
  ActivitiyType.FollowUp,
  ActivitiyType.Bootstrap,
  ActivitiyType.Keynote,
  ActivitiyType.Review,
  ActivitiyType.Surveillance
];

const modules = [
  "Cpe",
  "Psu",
  "Mul",
  "Mat",
  "Web",
  "Aia",
  "None"
];

export default function Home() {
  let [location, setLocation] = useState<string>('');
  let [asteks, setAsteks] = useState<number>(0);
  let [activitiyTypeIdx, setActivityTypeIdx] = useState<number>(0);
  let [moduleIdx, setModuleIdx] = useState<number>(0);

  let [result, setResult] = useState<Activity>();

  function handleChangeLocation(event: ChangeEvent<HTMLInputElement>) {
    setLocation(event.currentTarget.value)
  }

  function handleChangeAsteks(event: ChangeEvent<HTMLInputElement>) {
    setAsteks(parseInt(event.currentTarget.value))
  }

  function handleChangeActivityType(event: SelectChangeEvent<HTMLInputElement>) {
    setActivityTypeIdx(parseInt(event.target.value as string))
  }

  function handleChangeModule(event: SelectChangeEvent<HTMLInputElement>) {
    setModuleIdx(parseInt(event.target.value as string))
  }

  const ActivityPicker = (): ReactElement => {
    return (
      <div className="activity-picker">
        <SelectWrapper value={activitiyTypeIdx} label="Activity" onUpdate={handleChangeActivityType}>
          {activities}
        </SelectWrapper>
      </div>
    )
  }

  const ModulePicker = (): ReactElement => {
    return (
      <div className="module-picker">
        <SelectWrapper value={moduleIdx} label="Module" onUpdate={handleChangeModule}>
          {modules}
        </SelectWrapper>
      </div>
    )
  }

  function handleSubmit() {
    console.log(location, asteks, activities[activitiyTypeIdx], modules[moduleIdx]);
    createActivity(location, asteks, activities[activitiyTypeIdx], modules[moduleIdx]).then((response) => {
      setResult(response.data)
    })
  }


  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      <form noValidate autoComplete="off">
        <Box
          component="div"
          sx={{
            '& > :not(style)': { m: 1, width: '25ch' },
          }}
        >
          <TextField label="Location" margin="dense" focused onChange={handleChangeLocation} />
          <TextField defaultValue={0} type="number" label="Asteks" margin="dense" focused onChange={handleChangeAsteks} />
        </Box>
        <br></br>
        <ActivityPicker />
        <br></br>
        <ModulePicker />
        <br></br>
        <ButtonWrapper onClick={handleSubmit}>
          Create
        </ButtonWrapper>
        {JSON.stringify(result)}
      </form>
    </div>
  )
}
