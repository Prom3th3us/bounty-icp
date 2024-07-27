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

test.after(async () => {
  await server.stop()
})

// Test for /attach-bounty command on issue comment
test('receives /attach-bounty command on issue comment', async function () {
  const githubReceives = nock('https://api.github.com')
  const apiUrl = issueId => `/repos/client-org/client-repo/issues/${issueId}/comments`
  // When
  githubReceives
    .post(
      apiUrl(1),
      (requestBody) => {
        assert.equal(requestBody, { body: 'Creating bounty of $100 for issue #1' })
        return true
      }
    )
    .reply(201, {})
  // Given   
  await probot.receive({
    name: 'issue_comment',
    id: '1',
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
        body: '/bounty 100'
      }
    }
  })
  // Then
  assert.equal(mock.activeMocks(), [])
})

ignore('receives /bounty command on issue comment', async function () {
  const mock = nock('https://api.github.com')
    .post(
      '/repos/Prom3th3us/bounty-icp-gh-app/issues/1/comments',
      (requestBody) => {
        assert.equal(requestBody, { body: 'Creating bounty of $100 for issue #1' })
        return true
      }
    )
    .reply(201, {})

  await probot.receive({
    name: 'issue_comment',
    id: '1',
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
        body: '/bounty 100'
      }
    }
  })

  assert.equal(mock.activeMocks(), [])
})

// Test for /attempt command on issue comment
ignore('receives /attempt command on issue comment', async function () {
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
ignore('receives /approve command on PR comment', async function () {
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
ignore('receives /claim command on PR body', async function () {
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
