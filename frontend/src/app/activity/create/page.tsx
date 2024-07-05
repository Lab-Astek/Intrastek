"use client";

import { FC, ChangeEvent, ReactElement, useState } from 'react'
import FormControl from '@mui/material/FormControl';
import { ActivitiyType } from '@/types/activity';
import RadioWrapper from '@/components/inputs/radio'

type ActivityPickerProp = {
  onUpdate?: (arg0: ChangeEvent<HTMLInputElement>) => void
}

const ActivityPicker: FC<ActivityPickerProp> = ({onUpdate = undefined}): ReactElement => {
  let activities = [
    ActivitiyType.Permanence,
    ActivitiyType.FollowUp,
    ActivitiyType.Bootstrap,
    ActivitiyType.Keynote,
    ActivitiyType.Review,
    ActivitiyType.Surveillance
  ];

  return (
    <div className="activity-picker">
      <FormControl>
        <RadioWrapper defaultValue={activities[0]} name="activity" row onUpdate={onUpdate}>
          {activities.map((s, idx) => s)}
        </RadioWrapper>
      </FormControl>
    </div>
  )
}

export default function Home() {
  let [location, setLocation] = useState<string>('');
  let [asteks, setAsteks] = useState<number>(0);
  let [activitiyType, setActivityType] = useState<ActivitiyType>();

  async function onSubmit(event: ChangeEvent<HTMLFormElement>) {
    event.preventDefault()

    console.log(location)
    console.log(asteks)
  }

  function handleChangeLocation(event: ChangeEvent<HTMLInputElement>) {
    setLocation(event.currentTarget.value)
  }

  function handleChangeAsteks(event: ChangeEvent<HTMLInputElement>) {
    setAsteks(parseInt(event.currentTarget.value))
  }

  function onUpdateActivity(event: ChangeEvent<HTMLInputElement>) {
    setActivityType(event.currentTarget.value as ActivitiyType)
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      <form onSubmit={onSubmit}>
        <label htmlFor="location">Location</label><br></br>
        <input type="text" name="location" onChange={handleChangeLocation} /><br></br>

        <label htmlFor="asteks">Asteks</label><br></br>
        <input type="number" name="asteks" onChange={handleChangeAsteks} /><br></br>

        <ActivityPicker onUpdate={onUpdateActivity}/>
        <button type="submit">Submit</button>

        <br></br>
        <br></br>
        <h2>{activitiyType}, {location}, {asteks}</h2>
      </form>
    </div>
  )
}
