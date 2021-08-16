<div id='main' style="background-color: {$theme.backgroundColor}; font-family: {$theme.font}; color: {$theme.textColor}; font-size: {$theme.fontSize};">
  {#if showNewAccount}
    <div
      id="newAccountDialog"
      style="background-color: {$theme.backgroundColor}; color: {$theme.textColor};"
    >
      <label 
        for="accountDefaultInput"
        class='newAccountLabel'
      >
        Default:
      </label>
      <input 
        id="accountDefaultInput"
        type="checkbox"
        bind:checked={accountDefault}
      />
      <label 
        for="accountNameInput"
        class='newAccountLabel'
      >
        Name of Account:
      </label>
      <input 
        id="accountNameInput"
        bind:value={accountName}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      ></textarea>
      <div
        id="buttonrow"
        style="grid-column: 1 / span 2;"
      >
        <button 
          id="save"
          on:click={saveNewAccount}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          Save
        </button>
        <button 
          id="cancel"
          on:click={cancelNewAccount}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          Cancel
        </button>
      </div>
    </div>
  {/if}
  {#if showChangeAccount}
    <div
      id="accountsDiv"
      style="background-color: {$theme.backgroundColor}; color: {$theme.textColor};"
    >
      <h1>Email It - Change Account</h1>
      {#if $account !== undefined}
        <h2>Current Account: {$account.name}</h2>
      {:else}
        <h2>Current Account: Please Create an Account</h2>
      {/if}
      {#each accounts as acc}
        <button
          on:click={() => { changeActiveAccount(acc); }}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          Save
        </button>
        <button 
          id="new"
          on:click={newAccount}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          New
        </button>
        <button 
          id="edit"
          on:click={editAccountChange}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          Edit
        </button>
        <button 
          id="cancel"
          on:click={cancelAccountChange}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
        >
          Cancel
        </button>
        <button 
          id="delete"
          on:click={deleteAccount}
          style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
      <SimpleAutoComplete 
        inputId="receiverInput"
        items={emails} 
        labelFieldName="email"
        bind:selectedItem={receiver}
        inputClassName='receiverInput'
        className='receiverDiv'
        create=true
        theme={$theme}
        onBlur={val => {
          $email.to = receiver.value;
        }}
      />
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
        on:blur={saveEmailState}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      />
    </div>
  </div>
  {#if showPreview}
    <div 
      id='preview'
      style="border-color: {$theme.textColor};"
    >
      {@html previewHTML}
    </div>
  {:else}
    <CodeMirror 
      height='290px' 
      width='958px' 
      config={editorConfig}
      initFinished={initFinished}
      styling="position: relative; margin-bottom: 20px; border: solid 1px transparent; border-radius: 20px; overflow: hidden;"
      on:textChange='{(event) => {textChanged(event.detail.data)}}' 
      on:editorChange='{(event) => {editorChange(event.detail.data); }}'
    />
  {/if}
  <div
    id='buttonrow'
  >
    <button 
      on:click={viewNotes}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Notes
    </button>
    {#if emailState === 'edit'}
      <button 
        id="showPreview"
        on:click={createPreview}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      >
        Preview
      </button>
    {:else}
      <button 
        id="edit"
        on:click={editEmail}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      >
        Edit
      </button>
      <button 
        id="send"
        on:click={sendEmail}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      >
        Send
      </button>
    {/if}
    {#if ($account === undefined)}
      <button
        id="account"
        on:click={newAccount}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      >
        Create a New Account
      </button>
    {:else}
      <button
        id="account"
        on:click={changeAccount}
        style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
      >
        {$account.name}
      </button>
    {/if}
    <button 
      id="clear"
      on:click={clearEmail}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Clear
    </button>
    <button 
      on:click={viewTemplateMenu}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Templates
    </button>
    <button 
      on:click={viewScriptsMenu}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
    >
      Scripts
    </button>
    <button 
      id="cancel"
      on:click={cancelEmail}
      style="background-color: {$theme.textAreaColor}; color: {$theme.textColor}; border-color: {$theme.borderColor};"
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
    margin: 0px 5px;
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
  import { appWindow } from "@tauri-apps/api/window";
  import SimpleAutoComplete from "../components/SimpleAutocomplete.svelte";
  import CodeMirror from "../components/CodeMirror.svelte";
  import showdown from 'showdown';
  import { theme } from '../stores/theme.js';
  import { state } from '../stores/state.js';
  import { account } from '../stores/account.js';
  import { email } from '../stores/email.js';
  import { emailEditor } from '../stores/emailEditor.js';
  import { commandLineEmail } from '../stores/commandLineEmail.js'
  import { showScripts } from '../stores/showScripts.js';
  import { showTemplates } from '../stores/showTemplates.js';

  let receiver = {
      name: '',
      email: ''
    };
  let subject = '';
  let emailState = 'edit';
  let accounts;
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
  let accountDefault = false;
  let previewHTML = '';
  let bodyValue = '';
  let oldState = '';
  let starting = true;
  let emails = [];
  let editorConfig = {
    language: 'markdown',
    lineNumbers: false,
    lineWrapping: true,
    lineHighlight: true
  };
  let initFinished = true;


  onMount(()=>{
    getEmails();
    getAccounts();
    emailState = 'edit';
    oldState = 'edit';
    initFinished = true;
  });

  afterUpdate(() => {
    if(($commandLineEmail !== undefined)&&($commandLineEmail !== '')) {
      receiver = {
        name: '',
        email: $commandLineEmail
      };
      $email.to = $commandLineEmail;
      $commandLineEmail = undefined;
    } 
    if(starting) {
      console.log($email);
      console.log($emailEditor);
      receiver = {
        name: '',
        email: $email.to 
      };
      var rec = document.getElementById('receiverInput');
      rec.value = $email.to;
      subject.value = $email.subject;
      starting = false;
      $emailEditor.setValue($email.body);
    }
    if((emailState === 'edit')&&(oldState === 'preview')) {
      $emailEditor.setValue(bodyValue);
      oldState = 'edit';
    }
  });

  function editorChange(e) {
    $emailEditor = e;
  }

  function textChanged(textCursor) {
    $email.body = textCursor.value;
    bodyValue = textCursor.value;
  }

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
    origAccount = $account;
    showChangeAccount = true;
  }

  function saveAccount() {
    if(emailState === 'preview') {
      makeHtml();
    }
    showChangeAccount = false;
  }

  function clearFormData() {
    accountName = '';
    accountDefault = false;
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
    $account = origAccount;
    if(emailState === 'edit') {
      bodyValue = $emailEditor.getValue();
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
        if((accounts.length > 0)&&($account === undefined)) {
          $account = accounts.find(item => item.default);
        }
        if(typeof callback !== 'undefined') callback();
      });
  }

  function changeActiveAccount( acc ) {
    //
    // Set new account and create previews.
    //
    $account = acc;
    if(emailState === 'edit') {
      bodyValue = $emailEditor.getValue();
    } else {
      makeHtml();
    }
  }

  function createPreview() {
    var toAddress;
    if(typeof $email.to !== 'undefined') {
      toAddress = $email.to;
    } else {
      var em = document.getElementById('receiverInput').value;
      toAddress = em;
    }
    if(validate(toAddress)) {
   
      // 
      // Keep a copy of the body value for when we exit preview mode.
      //
      bodyValue = $emailEditor.getValue();

      // 
      // Set to preview and keep a copy of the new state.
      // 
      emailState = 'preview';
      oldState = emailState;

      // 
      // Creat a preview of the email.
      //
      makeHtml();

      //
      // Show the preview.
      // 
      showPreview = true;
    } else {
      alert("Invalid Email");
    }
  }

  function makeHtml() {
    var converter = new showdown.Converter({
      extensions: [
      ],
    });
    converter.setOption('tables',true);
    previewHTML = converter.makeHtml(bodyValue + $account.signiture);
    if(typeof $account.headerHTML !== 'undefined') {
      previewHTML = $account.headerHTML + previewHTML + $account.footerHTML;
    }
  }

  async function editEmail() {
    emailState = 'edit';
    showPreview = false;
  }

  function cleanTags(msg) {
    return msg.replace(/<hr>/gimu,'\n').replace(/<[/]*[^>]+>/gimu,'');
  }

  async function sendEmail() {
    // 
    // This will tell the server to send the email.
    //
    var toAddress;
    if(typeof $email.to !== 'undefined') {
      toAddress = $email.to;
    } else {
      var em = document.getElementById('receiverInput').value;
      toAddress = em;
    }
    if(validate(toAddress)) {
      addToEmails('',toAddress);
      var bodyText = bodyValue + cleanTags($account.signiture);
      showPreview = false;
      emailState = 'edit';
      fetch('http://localhost:9978/api/emailit/send', {
        method: 'PUT',
        headers: {
          'Content-type': 'application/json'
        },
        body: Body.json({
          acc: $account,
          to: toAddress,
          from: $account.from,
          subject: subject.value,
          text: bodyText,
          html: previewHTML
        })
      });
    } else {
      alert('Invalid Email');
    }
  }

  function validate(email) {
    const emailRegexp = new RegExp(
      /^[a-zA-Z0-9][\-_\.\+\!\#\$\%\&\'\*\/\=\?\^\`\{\|]{0,1}([a-zA-Z0-9][\-_\.\+\!\#\$\%\&\'\*\/\=\?\^\`\{\|]{0,1})*[a-zA-Z0-9]@[a-zA-Z0-9][-\.]{0,1}([a-zA-Z][-\.]{0,1})*[a-zA-Z0-9]\.[a-zA-Z0-9]{1,}([\.\-]{0,1}[a-zA-Z]){0,}[a-zA-Z0-9]{0,}$/i
    );

    return emailRegexp.test(email);
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
    receiver = {
      name: '',
      email: ''
    };
    subject.value = '';
    $emailEditor.setValue('');
    bodyValue = '';
    $email.to = '';
    $email.subject = '';
    $email.body = '';
    showPreview = false;
    emailState = 'edit';
    oldState = 'edit';
  }

  function saveNewAccount() {
    var acc = {
      name: accountName,
      default: accountDefault,
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

    //
    // If this is to be the default account, make sure all the others 
    // are false.
    //
    if(accountDefault) {
      accounts = accounts.map(item => {
        item.default = false;
        return(item);
      });
    }
    var orig = accounts.filter(item => item.name === acc.name);
    if(orig.length > 0) {
      accounts = accounts.filter(item => item.name !== acc.name);
      accounts.push(acc);
    }
    $account = acc;
    accounts = accounts;
    if(emailState === 'preview') makeHtml();
    clearFormData();
    showNewAccount = false;
  }

  function deleteAccount() {
    var acc = $account;
    accounts = accounts.filter(item => item.name !== acc.name);
    if(accounts.length > 0) {
      $account = accounts[0];
      origAccount = accounts[0];
      if(emailState === 'preview') makeHtml();
    } else {
      showNewAccount = false;
      $account = undefined;
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
    accountName = $account.name;
    accountDefault = $account.default;
    accountFrom = $account.from;
    accountUsername = $account.username;
    accountSmptServer = $account.smtpserver;
    accountPassword = $account.password;
    accountPort = $account.port;
    accountSig = $account.signiture;
    accountHeaderHTML = $account.headerHTML;
    accountFooterHTML = $account.footerHTML;
    showNewAccount = true;
  }

  function viewNotes() {
    saveEmailState();
    state.set('notes');
  }

  function saveEmailState() {
    var rec = document.getElementById('receiverInput');
    $email.to = rec.value;
    $email.body = $emailEditor.getValue();
    $email.subject = subject.value;
  }

  function viewScriptsMenu() {
    $showScripts = ! $showScripts;
  }

  function viewTemplateMenu() {
    $showTemplates = ! $showTemplates;
  }
</script>

