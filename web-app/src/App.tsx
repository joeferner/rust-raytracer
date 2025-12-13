import { Button, Container, Text, Title } from '@mantine/core';
import './App.css'

export function App() {
  return (
    <Container size="sm" style={{ marginTop: '50px' }}>
      <Title order={1}>Welcome to Mantine!</Title>
      <Text size="lg" mt="md">
        Your React app with Mantine is ready to go.
      </Text>
      <Button mt="xl" size="md">
        Click me!
      </Button>
    </Container>
  );
}

