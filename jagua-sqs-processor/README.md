# Jagua SQS Processor

SQS queue processor for jagua-rs SVG nesting service.

## Overview

This service listens to an SQS queue for SVG nesting requests, processes them using the jagua-rs nesting algorithm, and sends results to an output SQS queue.

## Environment Variables

The following environment variables are required:

- `INPUT_QUEUE_URL` - SQS queue URL for incoming requests
- `OUTPUT_QUEUE_URL` - SQS queue URL for sending results

Optional:
- `RUST_LOG` - Log level (default: `info`)
- `RUST_BACKTRACE` - Enable backtraces (default: `1`)

## AWS IAM Permissions

The service requires the following IAM permissions:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "sqs:ReceiveMessage",
        "sqs:DeleteMessage",
        "sqs:GetQueueAttributes"
      ],
      "Resource": "arn:aws:sqs:*:*:input-queue-name"
    },
    {
      "Effect": "Allow",
      "Action": [
        "sqs:SendMessage"
      ],
      "Resource": "arn:aws:sqs:*:*:output-queue-name"
    }
  ]
}
```

## Building the Docker Image

```bash
docker build -t jagua-sqs-processor:latest -f jagua-sqs-processor/Dockerfile .
```

## Running Locally

```bash
docker run -e INPUT_QUEUE_URL=https://sqs.region.amazonaws.com/account/input-queue \
           -e OUTPUT_QUEUE_URL=https://sqs.region.amazonaws.com/account/output-queue \
           -e AWS_ACCESS_KEY_ID=your-key \
           -e AWS_SECRET_ACCESS_KEY=your-secret \
           -e AWS_REGION=us-east-1 \
           jagua-sqs-processor:latest
```

## Deploying to AWS

### ECS (Elastic Container Service)

1. Build and push to ECR:
```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com
docker tag jagua-sqs-processor:latest <account-id>.dkr.ecr.us-east-1.amazonaws.com/jagua-sqs-processor:latest
docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/jagua-sqs-processor:latest
```

2. Create ECS task definition with environment variables and IAM role

### ECS Fargate Example Task Definition

```json
{
  "family": "jagua-sqs-processor",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "containerDefinitions": [
    {
      "name": "jagua-sqs-processor",
      "image": "<account-id>.dkr.ecr.us-east-1.amazonaws.com/jagua-sqs-processor:latest",
      "essential": true,
      "environment": [
        {
          "name": "INPUT_QUEUE_URL",
          "value": "https://sqs.region.amazonaws.com/account/input-queue"
        },
        {
          "name": "OUTPUT_QUEUE_URL",
          "value": "https://sqs.region.amazonaws.com/account/output-queue"
        },
        {
          "name": "RUST_LOG",
          "value": "info"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/jagua-sqs-processor",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ],
  "taskRoleArn": "arn:aws:iam::<account-id>:role/jagua-sqs-processor-task-role",
  "executionRoleArn": "arn:aws:iam::<account-id>:role/ecsTaskExecutionRole"
}
```

## Request Format

The service expects JSON messages in the following format:

```json
{
  "correlation_id": "unique-request-id",
  "svg_base64": "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0i....", 
  "bin_width": 350.0,
  "bin_height": 350.0,
  "spacing": 50.0,
  "amount_of_parts": 4,
  "amount_of_rotations": 8,
  "config": null,
  "output_queue_url": "https://sqs.region.amazonaws.com/account/output-queue"
}
```

## Response Format

The service sends JSON messages in the following format:

```json
{
  "correlation_id": "unique-request-id",
  "first_page_svg_base64": "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0i....",
  "last_page_svg_base64": "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0i....",
  "parts_placed": 2,
  "is_improvement": false,
  "is_final": true,
  "timestamp": 1234567890
}
```

* `first_page_svg_base64` - SVG for the first page/bin
* `last_page_svg_base64` - SVG for the last page/bin (omitted when no parts are placed)
* Intermediate responses (`is_improvement = true`) only include `first_page_svg_base64`
```


