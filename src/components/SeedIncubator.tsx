import { useState } from "react";
import { Button, Input, Text } from "@stellar/design-system";
import { useWallet } from "../hooks/useWallet";
import seedIncubator from "../contracts/seed_incubator";
import { Box } from "./layout/Box";

export const SeedIncubator = () => {
  const [newTemperature, setNewTemperature] = useState<number>();
  const { address } = useWallet();

  if (!address) {
    return (
      <Text as="p" size="md">
        Connect wallet to play the guessing game
      </Text>
    );
  }

  const submitTemperature = async () => {
    if (!newTemperature || !address) return;
    const { result } = await seedIncubator.update_temperature({
      new_temp: newTemperature,
    });
    console.log({ result });
    // if (result.isErr()) {
    // console.error(result.unwrapErr());
    // } else {
    //setGuessedIt(result.unwrap());
    // }
  };

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        void submitTemperature();
      }}
    >
      <Box gap="sm" direction="row" align="end" justify="end" wrap="wrap">
        <Input
          label="Set a new temperature"
          id="temperature"
          fieldSize="lg"
          onChange={(e) => {
            setNewTemperature(Number(e.target.value));
          }}
        />
        <Button
          type="submit"
          disabled={!newTemperature}
          style={{ marginTop: 8 }}
          variant="primary"
          size="md"
        >
          Submit Temperature
        </Button>
      </Box>

      <Text as="p" size="lg">
        &nbsp; {/* Not sure the SDS way to add consistent spacing at the end */}
      </Text>
    </form>
  );
};
