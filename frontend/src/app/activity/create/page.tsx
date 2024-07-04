"use client";

import { FormEvent, useState } from 'react'

export default function Home() {
  let [location, setLocation] = useState<string>('');
  let [asteks, setAsteks] = useState<number>(0);

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()

    console.log(location)
    console.log(asteks)
  }

  function handleChangeLocation(event: FormEvent<HTMLInputElement>) {
    setLocation(event.currentTarget.value)
  }

  function handleChangeAsteks(event: FormEvent<HTMLInputElement>) {
    setAsteks(parseInt(event.currentTarget.value))
  }

  function ActivityPicker() {
    return (
      <div>
        <label htmlFor="followup">FollowUp</label>
        <input type="radio" id="followup" name="activity" value="FollowUp" />
        <label htmlFor="bootstrap">Bootstrap</label>
        <input type="radio" id="bootstrap" name="activity" value="Bootstrap" />
        <label htmlFor="review">Review</label>
        <input type="radio" id="review" name="activity" value="Review" />
        <label htmlFor="keynote">Keynote</label>
        <input type="radio" id="keynote" name="activity" value="Keynote" />
        <label htmlFor="surveillance">Surveillance</label>
        <input type="radio" id="surveillance" name="activity" value="Surveillance" />
        <label htmlFor="permanence">Permanence</label>
        <input type="radio" id="permanence" name="activity" value="Permanence" />
      </div>
    )
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      <form onSubmit={onSubmit}>
        <label htmlFor="location">Location</label><br></br>
        <input type="text" name="location" onChange={handleChangeLocation} /><br></br>

        <label htmlFor="asteks">Asteks</label><br></br>
        <input type="number" name="asteks" onChange={handleChangeAsteks} /><br></br>

        <ActivityPicker />
        <button type="submit">Submit</button>
      </form>
    </div>
  )
}
