# Walk The World Backend

This is a backend for a trivial web application: the idea is for users to add their daily steps, which is aggregated towards the goal of walking around the world.

This served as a nice first project for working with Rust and serverless infrastructure.

## Features

- 🦀 Rust 🙏
- 🔓 Cognito for login and identity management
- 🚀 API Gateway REST API (with request body validation to prevent failed lambda invocations)
- 🧨 DynamoDb
- 🛠 Staged deployments for everything!

## 📦 Development

Install the [serverless framework](https://serverless.com/framework/) cli.

Run `npm ci`, which will make sure npm dependencies are installed based directly on your package-lock.json file. This only needs run once.
The first time you run `npx serverless deploy` it will pull down and compile the base set
of dependencies and your application. Unless the dependencies change afterwards,
this should only happen once, resulting in an out of the box rapid deployment
cycle.

## 🛵 Continuous integration and deployment

TODO using Github actions

## Local deployment

```bash
serverless deploy -v
```

The `-v` is important as this will print out all the created resources from cloudformation. These resources are then environment variables for the frontend.

### Creating a user

Either use a real signup flow or create a user using the aws cli

```bash
aws cognito-idp sign-up \
  --region YOUR_COGNITO_REGION \
  --client-id YOUR_COGNITO_APP_CLIENT_ID \
  --username admin@example.com \
  --password Passw0rd!
```

And then verify the user

```bash
aws cognito-idp admin-confirm-sign-up \
  --region YOUR_COGNITO_REGION \
  --user-pool-id YOUR_COGNITO_USER_POOL_ID \
  --username admin@example.com
```

## 🔬 Logs

With your function deployed you can now tail it's logs right from your project

```sh
$ npx serverless logs -f $FUNCTION_NAME
```

## 👴 Retiring

Good code should be easily replaceable. Good code is should also be easily disposable. Retiring applications should be as easy as creating and deploying them them. The dual of `serverless deploy` is `serverless remove`. Use this for retiring services and cleaning up resources.

```bash
$ npx serverless remove
```

## ℹ️ Additional information

This was bootstrapped from [this Rust serverless template](https://github.com/softprops/serverless-aws-rust-multi).

- See the [serverless-rust plugin's documentation](https://github.com/softprops/serverless-rust) for more information on plugin usage.

- See the [aws rust runtime's documentation](https://github.com/awslabs/aws-lambda-rust-runtime) for more information on writing Rustlang lambda functions
- Much of this infra was inspired by [serverless-stack](https://serverless-stack.com)
