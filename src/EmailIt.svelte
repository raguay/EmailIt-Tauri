<div id='main' style="background-color: {theme.backgroundColor}; font-family: {theme.font}; color: {theme.textColor}; font-size: {theme.fontSize};">
  {#if showNewAccount}
    <div
      id="newAccountDialog"
      style="background-color: {theme.backgroundColor}; color: {theme.textColor};"
    >
      <label 
        for="accountNameInput"
        class='newAccountLabel'
      >
        Name of Account:
      </label>
      <input 
        id="accountNameInput"
        bind:value={accountName}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountFromInput"
        class='newAccountLabel'
      >
        From Email:
      </label>
      <input 
        id="accountFromInput"
        bind:value={accountFrom}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountSmptServerInput"
        class='newAccountLabel'
      >
        Address of SMPT Server:
      </label>
      <input 
        id="accountSmptServerInput"
        bind:value={accountSmptServer}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountPortInput"
        class='newAccountLabel'
      >
        SMPT Server Port:
      </label>
      <input 
        id="accountPortInput"
        bind:value={accountPort}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountUsernameInput"
        class='newAccountLabel'
      >
        User Name:
      </label>
      <input 
        id="accountUsernameInput"
        bind:value={accountUsername}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountPasswordInput"
        class='newAccountLabel'
      >
        Password:
      </label>
      <input 
        id="accountPasswordInput"
        bind:value={accountPassword}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
      <label 
        for="accountSigInput"
        class='newAccountLabel'
      >
        Signiture:
      </label>
      <textarea
        id="accountSigInput"
        bind:value={accountSig}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      ></textarea>
      <label 
        for="accountHeaderHTMLInput"
        class='newAccountLabel'
      >
        Header HTML:
      </label>
      <textarea
        id="accountHeaderHTMLInput"
        bind:value={accountHeaderHTML}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      ></textarea>
      <label 
        for="accountFooterHTMLInput"
        class='newAccountLabel'
      >
        Footer HTML:
      </label>
      <textarea
        id="accountFooterHTMLInput"
        bind:value={accountFooterHTML}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      ></textarea>
      <div
        id="buttonrow"
        style="grid-column: 1 / span 2;"
      >
        <button 
          id="save"
          on:click={saveNewAccount}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Save
        </button>
        <button 
          id="cancel"
          on:click={cancelNewAccount}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Cancel
        </button>
      </div>
    </div>
  {/if}
  {#if showChangeAccount}
    <div
      id="accountsDiv"
      style="background-color: {theme.backgroundColor}; color: {theme.textColor};"
    >
      <h1>Email It - Change Account</h1>
      {#if currentAccount !== undefined}
        <h2>Current Account: {currentAccount.name}</h2>
      {:else}
        <h2>Current Account: Please Create an Account</h2>
      {/if}
      {#each accounts as acc}
        <button
          on:click={() => { changeActiveAccount(acc); }}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          {acc.name}
        </button>
      {/each}
      <div
        id='buttonrow'
      >
        <button 
          id="save"
          on:click={saveAccount}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Save
        </button>
        <button 
          id="new"
          on:click={newAccount}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          New
        </button>
        <button 
          id="edit"
          on:click={editAccountChange}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Edit
        </button>
        <button 
          id="cancel"
          on:click={cancelAccountChange}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Cancel
        </button>
        <button 
          id="delete"
          on:click={deleteAccount}
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
        >
          Delete
        </button>
      </div>
    </div>
  {/if}
  <h1>Email It</h1>
  <div 
    id='header'
  >
    <div
      class='headerRow'
    >
      <label
        for='receiverInput'
      >
        To:
      </label>
      {#if emails !== undefined}
        <SimpleAutoComplete 
          inputId="receiverInput"
          items={emails} 
          labelFieldName="email"
          bind:selectedItem={receiver}
          inputClassName='receiverInput'
          className='receiverDiv'
          create=true
          theme={theme}
        />
      {:else}
        <input 
          id='receiverInput' 
          bind:this={receiver}
          type='email'
          multiple
          style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
          on:blur={(e)=>{ receiver.checkValidity(); }}
        />
      {/if}
    </div>
    <div
      class='headerRow'
    >
      <label
        for='subject'
      >
        Subject:
      </label>
      <input
        id='subject'
        bind:this={subject}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      />
    </div>
  </div>
  {#if showPreview}
    <div 
      id='preview'
      style="border-color: {theme.textColor};"
    >
      {@html previewHTML}
    </div>
  {:else}
    <textarea
      id='body'
      bind:this={emailbody}
      style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
    ></textarea>
  {/if}
  <div
    id='buttonrow'
  >
    {#if state === 'edit'}
      <button 
        id="showPreview"
        on:click={createPreview}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      >
        Preview
      </button>
    {:else}
      <button 
        id="edit"
        on:click={editEmail}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      >
        Edit
      </button>
      <button 
        id="send"
        on:click={sendEmail}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      >
        Send
      </button>
    {/if}
    {#if (currentAccount === undefined)}
      <button
        id="account"
        on:click={newAccount}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      >
        Create a New Account
      </button>
    {:else}
      <button
        id="account"
        on:click={changeAccount}
        style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
      >
        {currentAccount.name}
      </button>
    {/if}
    <button 
      id="clear"
      on:click={clearEmail}
      style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
    >
      Clear
    </button>
    <button 
      id="cancel"
      on:click={cancelEmail}
      style="background-color: {theme.textAreaColor}; color: {theme.textColor}; border-color: {theme.borderColor};"
    >
      Close
    </button>
  </div>
</div>

<style>
  :global(body) {
    margin: 0px;
    padding: 0px;
    background-color: black;
    overflow: hidden;
  }
  
  :global(table) {
    border-collapse: separate;
    border-spacing: 20px 5px;
  }

  #main {
    display: flex;
    flex-direction: column;
    margin: 0px;
    padding: 20px;
  }

  .headerRow {
    display: flex;
    flex-direction: row;
    max-height: 40px;
    margin: 0px 0px 20px 0px;
    width: 100%;
  }

  #buttonrow {
    display: flex;
    flex-direction: row;
    margin: auto;
  }

  #buttonrow button {
    border-radius: 10px;
    padding: 5px 20px 5px 20px;
    margin: 0px 20px;
    width: 100%;
    max-height: 40px;
    height: 40px;
    width: auto;
  }

  #header {
    display: flex;
    flex-direction: column;
    margin: 0px 0px 0px 0px;
    width: 100%;
  }

  #accountsDiv {
    display: flex;
    flex-direction: column;
    z-index: 100;
    position: absolute;
    width: 80%;
    padding: 20px;
    border-radius: 10px;
    top: 10%;
    left: 10%;
    background-color: lightblue;
  }

  #accountsDiv button {
    border-radius: 20px;
  }

  #newAccountDialog {
    display: grid;
    grid-template-columns: 1fr 2fr;
    grid-gap: 20px;
    width: 80%;
    z-index: 200;
    position: absolute;
    top: 10px;
    left: 10%;
    background: aliceblue;
    border-radius: 10px;
    padding: 20px;
    height: 85%;
    overflow-y: auto;
  }

  #newAccountDialog textarea {
    height: 100px;
  }

  #preview {
    height: 270px;
    max-height: 270px;
    border-radius: 20px;
    overflow-y: auto;
    padding: 10px;
    border: solid 3px;
    border-radius: 20px;
    margin: 0px 0px 15px 0px;
  }

  :global(.receiverInput) {
    margin: 0px;
    padding: 0px;
    width: 100%;
    border-radius: 10px;
    border-color: transparent;
  }

  :global(.input-container) {
    background-color: transparent !important;
    width: 100% !important;
    padding: 0px !important;
    margin: 0px !important;
    border-color: transparent !important;
  }

  :global(.receiverDiv) {
    background-color: transparent;
    width: 100%;
  }

  .newAccountLabel {
    width: 310px;
  }

  label {
    display: grid;
    justify-content: right;
    margin: auto 20px auto 0px;
    width: 150px;
  }

  input {
    margin: 0px;
    padding: 8px;
    width: 100%;
  }

  h1 {
    margin: 20px auto;
  }

  textarea {
    height: 300px;
    max-height: 300px;
    border-radius: 20px;
    padding: 10px;
  }

  input {
    border-radius: 10px;
  }
</style>

<script>
  import { afterUpdate, onMount } from 'svelte';
  import { fetch, Body } from "@tauri-apps/api/http";
  import { exit } from "@tauri-apps/api/process";
  import { getMatches } from "@tauri-apps/api/cli";
  import { appWindow } from "@tauri-apps/api/window";
  import SimpleAutoComplete from "./components/SimpleAutocomplete.svelte";
  import showdown from 'showdown';

  let receiver = '';
  let subject = '';
  let emailbody = '';
  let state = 'edit';
  let accounts;
  let currentAccount;
  let showChangeAccount = false;
  let showNewAccount = false;
  let showPreview = false;
  let origAccount;
  let accountFrom = '';
  let accountName = '';
  let accountUsername = '';
  let accountSmptServer = '';
  let accountPassword = '';
  let accountPort = '';
  let accountSig = '';
  let accountHeaderHTML = '';
  let accountFooterHTML = '';
  let previewHTML = '';
  let bodyValue = '';
  let oldState = '';
  let starting = true;
  let emails = undefined;
  let theme = {
    font: "Fira Code, Menlo",
    fontSize: "16pt",
    textAreaColor: '#454158',
    backgroundColor: '#22212C',
    textColor: '#80ffea',
    borderColor: '#1B1A23',
    Cyan: "#80FFEA",
    Green: "#8AFF80",
    Orange: "#FFCA80",
    Pink: "#FF80BF",
    Purple: "#9580FF",
    Red: "#FF9580",
    Yellow: "#FFFF80"
  };
  let socket;
  let commandLineEmail;

  onMount(()=>{
    getEmails();
    getMatches().then((matches) => {
      if(typeof matches.args.emails.value !== 'undefined') {
        commandLineEmail = matches.args.emails.value;
      }
    });
    getAccounts();
    state = 'edit';
    oldState = 'edit';
  });

  afterUpdate(() => {
    if((state === 'edit')&&(oldState === 'preview')) {
      emailbody.value = bodyValue;
      oldState = 'edit';
    }
    if(starting) {
      starting = false;
      appWindow.show();
    }
    if((typeof commandLineEmail !== 'undefined')&&(typeof emails !== 'undefined')) {
      if(receiver === null) {
        document.getElementById('receiverInput').value = commandLineEmail;
      } else {
        receiver.value = commandLineEmail;
      }
      commandLineEmail = undefined;
    }
  });

  function getEmails(callback) {
    // 
    // Get the emails from the server.
    // 
    fetch('http://localhost:9978/api/emailit/emails', {
        method: 'GET',
        headers: {
          'Content-type': 'application/json'
        }
      }).then(resp => {
        return resp.data;
      })
      .then(data => {
        emails = data.emails;
        if(typeof callback !== 'undefined') callback();
      });
  }

  function changeAccount() {
    origAccount = currentAccount;
    showChangeAccount = true;
  }

  function saveAccount() {
    if(state === 'preview') {
      makeHtml();
    }
    showChangeAccount = false;
  }

  function clearFormData() {
    accountName = '';
    accountFrom = '';
    accountUsername = '';
    accountSmptServer = '';
    accountPassword = '';
    accountPort = '';
    accountSig = '';
    accountHeaderHTML = '';
    accountFooterHTML = '';
  }
  
  function newAccount() {
    clearFormData();
    showNewAccount = true;
  }

  function cancelAccountChange() {
    currentAccount = origAccount;
    if(state === 'edit') {
      bodyValue = emailbody.value;
    } else {
      makeHtml();
    }
    showChangeAccount = false;
  }

  function getAccounts(callback) {
    // 
    // Get the accounts from the server.
    // 
    fetch('http://localhost:9978/api/emailit/accounts', {
        method: 'GET',
        headers: {
          'Content-type': 'application/json'
        }
      }).then(resp => {
        return resp.data;
      })
      .then(accs => {
        accounts = accs;
        if(accounts.length > 0) currentAccount = accounts[0];
        if(typeof callback !== 'undefined') callback();
      });
  }

  function changeActiveAccount( acc ) {
    //
    // Set new account and create previews.
    //
    currentAccount = acc;
    if(state === 'edit') {
      bodyValue = emailbody.value;
    } else {
      makeHtml();
    }
  }

  function createPreview() {
    // 
    // Keep a copy of the body value for when we exit preview mode.
    //
    bodyValue = emailbody.value;

    // 
    // Set to preview and keep a copy of the new state.
    // 
    state = 'preview';
    oldState = state;

    // 
    // Creat a preview of the email.
    //
    makeHtml();

    //
    // Show the preview.
    // 
    showPreview = true;
  }

  function makeHtml() {
    var converter = new showdown.Converter({
      extensions: [
      ],
    });
    converter.setOption('tables',true);
    previewHTML = converter.makeHtml(bodyValue + currentAccount.signiture);
    if(typeof currentAccount.headerHTML !== 'undefined') {
      previewHTML = currentAccount.headerHTML + previewHTML + currentAccount.footerHTML;
    }
  }

  async function editEmail() {
    state = 'edit';
    showPreview = false;
  }

  function cleanTags(msg) {
    return msg.replace(/<hr>/gimu,'\n').replace(/<[/]*[^>]+>/gimu,'');
  }

  async function sendEmail() {
    // 
    // This will tell the server to send the email.
    //
    var bodyText = bodyValue + cleanTags(currentAccount.signiture);
    showPreview = false;
    state = 'edit';
    var toAddress;
    if(receiver === null) {
      receiver = document.getElementById('receiverInput').value;
      toAddress = receiver;
      addToEmails('', receiver);
    } else if(emails === undefined) {
      toAddress = receiver.value;
      addToEmails('', receiver.value);
    } else {
      toAddress = `${receiver.name} <${receiver.email}>`;
      addToEmails(receiver.name, receiver.email)
    }

    console.log(receiver);
        fetch('http://localhost:9978/api/emailit/send', {
        method: 'PUT',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json({
          acc: currentAccount,
          to: toAddress,
          from: currentAccount.from,
          subject: subject.value,
          text: bodyText,
          html: previewHTML
        })
      });
  }

  function addToEmails(name, email) {
    emails = emails.filter(item => item.email !== email);
    emails.push({
      name: name,
      email: email
    });
    emails = emails;
    fetch('http://localhost:9978/api/emailit/addEmail', {
        method: 'PUT',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json({
          name: name,
          email: email
        })
      });
  }

  function cancelEmail() {
    clearEmail();

    // 
    // TODO: hide the window.
    //
    exit();
  }

  function clearEmail() {
    if(receiver === null) {
      receiver = document.getElementById('receiverInput').value;
      receiver.value = '';
    } else if(emails === undefined) {
      receiver.value = '';
    } else {
      receiver = '';
    }
    subject.value = '';
    emailbody.value = '';
    bodyValue = '';
    showPreview = false;
    state = 'edit';
    oldState = 'edit';
  }

  function saveNewAccount() {
    var acc = {
      name: accountName,
      from: accountFrom,
      username: accountUsername,
      smtpserver: accountSmptServer,
      port: accountPort,
      password: accountPassword,
      signiture: accountSig,
      headerHTML: accountHeaderHTML,
      footerHTML: accountFooterHTML
    };
    saveNewAccountServer(acc);
    var orig = accounts.filter(item => item.name === acc.name);
    if(orig.length > 0) {
      accounts = accounts.filter(item => item.name !== acc.name);
      accounts.push(acc);
    }
    currentAccount = acc;
    accounts = accounts;
    if(state === 'preview') makeHtml();
    clearFormData();
    showNewAccount = false;
  }

  function deleteAccount() {
    var acc = currentAccount;
    accounts = accounts.filter(item => item.name !== acc.name);
    if(accounts.length > 0) {
      currentAccount = accounts[0];
      origAccount = accounts[0];
      if(state === 'preview') makeHtml();
    } else {
      showNewAccount = false;
      currentAccount = undefined;
      origAccount = undefined;
    }
    deleteAccountServer(acc);
  }

  function saveNewAccountServer( acc ) {
    // 
    // This will save the new account to the server.
    //
    fetch('http://localhost:9978/api/emailit/accounts', {
        method: 'PUT',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json(acc)
      });
  }

  function deleteAccountServer(acc) {
    // 
    // This will remove an account from the server.
    //
    fetch('http://localhost:9978/api/emailit/accounts', {
        method: 'DELETE',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json(acc)
      });
  }

  function cancelNewAccount() {
    clearFormData();
    showNewAccount = false;
  }

  function editAccountChange() {
    accountName = currentAccount.name;
    accountFrom = currentAccount.from;
    accountUsername = currentAccount.username;
    accountSmptServer = currentAccount.smtpserver;
    accountPassword = currentAccount.password;
    accountPort = currentAccount.port;
    accountSig = currentAccount.signiture;
    accountHeaderHTML = currentAccount.headerHTML;
    accountFooterHTML = currentAccount.footerHTML;
    showNewAccount = true;
  }
</script>

