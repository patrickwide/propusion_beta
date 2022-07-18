import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Propulsion } from "../target/types/propulsion";
import { expect, assert, use } from "chai";
import { BN } from "bn.js";

describe("propulsion", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const propulsion = anchor.workspace.Propulsion as Program<Propulsion>;
  const author = provider.wallet;

  const project_1_Keypair = anchor.web3.Keypair.generate();
  it('Createproject!', async () => {
    await propulsion.methods.createProject(
      "Hello, world",
      "Hello worlrd!,from test 0",
    ).accounts({
      projectData: project_1_Keypair.publicKey,
      author: author.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([project_1_Keypair]).rpc();

    let project_1 = await propulsion.account.projectData.fetch(
      project_1_Keypair.publicKey
    );

    console.log(project_1);

  });

  const proposal_1_Keypair = anchor.web3.Keypair.generate();
  it('Create proposal!', async () => {
    await propulsion.methods.createProposal(
      "proposal title here",
      "proposal description here",
      "2000",
      "2000",
      3000000000,
      5667665457,
      true,
      ["Yes", "No"],
      [
        {
          choice_id: 0,
          program_id: 2,
          args: [
            "foo",
            "bar"
          ],
        },
        {
          choice_id: 1,
          program_id: 2,
          args: [
            "foo",
            "bar"
          ],
        }
      ],
    ).accounts({
      proposalData: proposal_1_Keypair.publicKey,
      projectData: project_1_Keypair.publicKey,
      author: author.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([proposal_1_Keypair]).rpc();

    let proposal_1 = await propulsion.account.proposalData.fetch(
      proposal_1_Keypair.publicKey
    );

    console.log(proposal_1);
  });

  const vote_1_Keypair = anchor.web3.Keypair.generate();
  it('cast vote!', async () => {
    await propulsion.methods.castVote(
      [0],
      "Vote note here",
    ).accounts({
      voteData: vote_1_Keypair.publicKey,
      proposalData: proposal_1_Keypair.publicKey,
      projectData: project_1_Keypair.publicKey,
      author: author.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([vote_1_Keypair]).rpc();

    let vote_1 = await propulsion.account.voteData.fetch(
      vote_1_Keypair.publicKey
    );

    console.log(vote_1);
  });

  it("Trigger proposal by Adding member!", async () => {
    await propulsion.methods.triggerProposal().accounts({
      proposalData: proposal_1_Keypair.publicKey,
      projectData: project_1_Keypair.publicKey,
      author: author.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([]).rpc();

    let member_1 = await propulsion.account.projectData.fetch(
      project_1_Keypair.publicKey
    );
    console.log(member_1);
  });

  it("Trigger proposal by Removing member!", async () => {
    await propulsion.methods.triggerProposal().accounts({
      proposalData: proposal_1_Keypair.publicKey,
      projectData: project_1_Keypair.publicKey,
      author: author.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([]).rpc();

    let member_1 = await propulsion.account.projectData.fetch(
      project_1_Keypair.publicKey
    );
    console.log(member_1);
  });

});
