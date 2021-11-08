import "antd/dist/antd.css";
import { DatePicker, message } from "antd";
import React, { useState } from "react";
import moment, { Moment } from "moment";

export const Comp = ({ msg }: { msg: string }) => {
    const [date, setDate] = useState(moment());

  const handleChange = (value: Moment) => {
    message.info(
        `[${msg}] Selected Date: ${value ? value.format("YYYY-MM-DD") : "None"}`
    );
    setDate(value);
  };
  return (
    <>
      <DatePicker onChange={handleChange} />
      <div style={{ marginTop: 16 }}>
        Selected Date: {date ? date.format("YYYY-MM-DD") : "None"}
      </div>
    </>
  );
};

export const fun = () => 2;
