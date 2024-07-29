import { suite } from 'uvu'
import * as assert from 'uvu/assert'

import nock from 'nock'
nock.disableNetConnect()

import { Server, Probot, ProbotOctokit } from 'probot'

import app from '../main/app.js'

let probot
let server
const test = suite('app')
async function startServer() {
  server = new Server({
    port: 8080,
    Probot: Probot.defaults({
      // simple authentication as alternative to appId/privateKey
      githubToken: 'test',
      // disable logs
      logLevel: 'warn',
      // disable request throttling and retries
      Octokit: ProbotOctokit.defaults({
        throttle: { enabled: false },
        retry: { enabled: false }
      })
    })
  })

  await server.load(app);

  server.start();

  probot = server.probotApp;
}

test.before(async () => {
  await startServer()
})

test.before.each(() => {
  nock.cleanAll(); // Limpia todos los mocks antes de cada test
});

test.after(async () => {
  await server.stop()
})

// TODO!: Test border cases like: 
    // - El comentario dice /bounty-attached
    // - El comentario dice /bounty -100
    // - El comentario dice /bounty "whatever"
    // - El comentario dice /bounty 100 "whatever"

// Test for /bounty command on issue comment
test('receives /bounty command on issue comment', async function () {
  const githubReceives = nock('https://api.github.com')
  const issueId = 1
  const commentId = 2
  const clientOrg = "client-org" 
  const clientRepo = "client-repo"
  const reactionCommentApiUrl = `/repos/${clientOrg}/${clientRepo}/issues/comments/${commentId}/reactions`
  const issueCommentApiUrl = `/repos/${clientOrg}/${clientRepo}/issues/${issueId}/comments`
  const bountyAmount = 100
  const bountyDepositLink = "TODO!"
  
  // Variable para verificar el orden de las solicitudes
  let callOrder = 0

  // Given
  githubReceives
    .post(
      reactionCommentApiUrl,
      requestBody => {
        // TODO!: Check request headers
        assert.equal(JSON.stringify(requestBody), JSON.stringify({ content: "+1" }));
        assert.is(callOrder, 0, 'reactionCommentApiUrl should be called first');
        callOrder++
        return true
      }
    )
    .reply(201,{})

  githubReceives
    .post(
      issueCommentApiUrl,
      requestBody => {
        // TODO!: Check request headers
        assert.equal(JSON.stringify(requestBody), JSON.stringify({ 
          body: `Bounty of ${bountyAmount} created. Awaiting for deposit at ${bountyDepositLink}`
        }));
        assert.is(callOrder, 1, 'issueCommentApiUrl should be called second');
        callOrder++;
        return true
      }
    )
    .reply(201, {})
  
  // When   
  await probot.receive({
    name: 'issue_comment',
    id: commentId,
    payload: {
      action: 'created',
      repository: {
        owner: {
          //TODO!: Check if another user without permissions can attach bounty
          login: clientOrg
        },
        name: clientRepo
      },
      issue: {
        number: issueId,
        pull_request: null
      },
      comment: {
        body: '/bounty 100'
      }
    }
  })
  // Then
  // assert.equal(mock.activeMocks(), [])
  assert.is(nock.pendingMocks().length, 0, 'Not all nock interceptors were used!')
  assert.is(callOrder, 2, 'Both API calls should have been made')
})

// Test for /attempt command on issue comment
test.skip('receives /attempt command on issue comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Started working on attempt #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'issue_comment',
    id: '2',
    payload: {
      action: 'created',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      issue: {
        number: 1,
        pull_request: null
      },
      comment: {
        body: '/attempt #1'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /approve command on PR comment
test.skip('receives /approve command on PR comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Marking claim as ready to pay' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'pull_request_review_comment',
    id: '3',
    payload: {
      action: 'created',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      pull_request: {
        number: 1
      },
      comment: {
        body: '/approve'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /claim command on PR body
test.skip('receives /claim command on PR body', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Submitting claim for bounty of issue #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'pull_request',
    id: '4',
    payload: {
      action: 'opened',
      repository: {
        owner: {
          login: 'Prom3th3us'
        },
        name: 'bounty-icp-gh-app'
      },
      pull_request: {
        number: 1,
        body: '/claim #1'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

test.run()
