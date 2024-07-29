import * as express from 'express'
import fetch from 'isomorphic-fetch'
import { HttpAgent } from '@dfinity/agent'
import { identity } from './identity.js'
import { canisterId, createActor } from '../../../../src/declarations/backend/index.js'

// Require syntax is needed for JSON file imports
import { createRequire } from 'node:module'
const require = createRequire(import.meta.url)
// For the sake of this example, which will focus on local development, 
// you will simply read it from the local canister_ids.json file.
// TODO: Install dotenv and configure it to read from hidden .env
const localCanisterIds = require('../../../../.dfx/local/canister_ids.json')

// Use `process.env` if available provoded, or fall back to local
const effectiveCanisterId =
  canisterId?.toString() ?? localCanisterIds.backend.local

const agent = new HttpAgent({
  identity: await identity,
  // mainnet: https://icp-api.io
  host: 'http://127.0.0.1:4943',
  fetch
})

const actor = createActor(effectiveCanisterId, { agent })

/**
 * @param {import('probot').Probot} app
 * @param {import('probot').ApplicationFunctionOptions} options
 */
const bountyApp = (app, options) => {
  app.log.info('Yay! The app was loaded!')

  // Listen for commands on issue comments
  app.on('issue_comment.created', async (context) => {
    const command = context.payload.comment.body.trim()
    const issue = context.payload.issue
    const isPullRequest = !!context.payload.issue.pull_request

    // Check if the comment starts with one of the specified commands
    if (command.startsWith('/bounty') && !isPullRequest) {
      // Handle bounty command
      const bountyAmount = command.split(' ')[1]
      await context.octokit.reactions.createForIssueComment({
        owner: context.payload.repository.owner.login,
        repo: context.payload.repository.name,
        comment_id: context.id,
        content: '+1' // La reacción que quieres agregar
      });
      //FIXME: call cannisters
      const bountyDepositLink = "TODO!"
      const commentBody = `Bounty of ${bountyAmount} created. Awaiting for deposit at ${bountyDepositLink}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    } else if (command.startsWith('/attempt') && !isPullRequest) {
      // Handle attempt command
      const attemptNumber = command.split(' ')[1]
      const commentBody = `Started working on attempt ${attemptNumber}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Listen for commands on PR comments
  app.on('pull_request_review_comment.created', async (context) => {
    const command = context.payload.comment.body.trim()

    // Check if the comment is the /approve command
    if (command === '/approve') {
      // Handle approve command
      const commentBody = 'Marking claim as ready to pay'
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Listen for commands on PR body
  app.on('pull_request.opened', async (context) => {
    const command = context.payload.pull_request.body.trim()

    // Check if the PR body contains the /claim command
    if (command.startsWith('/claim')) {
      // Handle claim command
      const issueNumber = command.split(' ')[1]
      const commentBody = `Submitting claim for bounty of issue ${issueNumber}`
      await context.octokit.issues.createComment(context.issue({ body: commentBody }))
      app.log.info(commentBody)
    }
  })

  // Log any event that the app receives
  app.onAny(async (context) => {
    app.log.info({ event: context.name, payload: context.payload })
  })

  // Log errors
  app.onError(async (error) => {
    app.log.error(error)
  })

  // Get an express router to expose new HTTP endpoints
  const router = options.getRouter('/bounty')

  // Use any middleware
  router.use(express.static('public'))

  // Add a new route
  router.get('/healthcheck', async (req, res) => {
    let result = await actor.healthcheck()
    res.send(result)
  })
}

export default bountyApp

