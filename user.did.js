export const idlFactory = ({ IDL }) => {
  const Args = IDL.Record({
    'answer_content' : IDL.Text,
    'question_id' : IDL.Text,
    'answer_pid' : IDL.Principal,
  });
  const Response = IDL.Variant({
    'Success' : IDL.Null,
    'AnswerInvalid' : IDL.Null,
  });
  const Args_1 = IDL.Record({
    'comment_content' : IDL.Text,
    'question_id' : IDL.Text,
    'answer_pid' : IDL.Principal,
    'comment_pid' : IDL.Principal,
  });
  const Args_2 = IDL.Record({
    'question_title' : IDL.Text,
    'question_asker' : IDL.Principal,
    'question_logo' : IDL.Opt(IDL.Text),
    'question_description' : IDL.Text,
    'tags' : IDL.Vec(IDL.Text),
    'question_image' : IDL.Opt(IDL.Text),
  });
  const Response_1 = IDL.Variant({
    'QuestionInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_3 = IDL.Record({
    'user' : IDL.Principal,
    'question_id' : IDL.Text,
  });
  const Response_2 = IDL.Variant({
    'Success' : IDL.Null,
    'PrincipalInvalid' : IDL.Null,
  });
  const Args_4 = IDL.Record({ 'question_id' : IDL.Text });
  const Response_3 = IDL.Variant({
    'DownVoteInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_5 = IDL.Record({
    'owner' : IDL.Principal,
    'to_follow' : IDL.Principal,
  });
  const Comment = IDL.Record({
    'comment_content' : IDL.Text,
    'comment_date' : IDL.Text,
    'comment_pid' : IDL.Text,
    'up_comment' : IDL.Nat32,
  });
  const Answer = IDL.Record({
    'answer_content' : IDL.Text,
    'up_thumb' : IDL.Nat32,
    'answer_date' : IDL.Text,
    'comments' : IDL.Vec(IDL.Tuple(IDL.Text, Comment)),
    'answer_pid' : IDL.Text,
  });
  const SuccessResult = IDL.Record({ 'answer_list' : IDL.Vec(Answer) });
  const Response_4 = IDL.Variant({ 'Success' : SuccessResult });
  const Args_6 = IDL.Record({
    'question_id' : IDL.Text,
    'answer_pid' : IDL.Principal,
  });
  const SuccessResult_1 = IDL.Record({ 'comment_list' : IDL.Vec(Comment) });
  const Response_5 = IDL.Variant({ 'Success' : SuccessResult_1 });
  const SuccessResult_2 = IDL.Record({
    'question_id_list' : IDL.Vec(IDL.Text),
  });
  const Response_6 = IDL.Variant({ 'Success' : SuccessResult_2 });
  const Question = IDL.Record({
    'question_title' : IDL.Text,
    'question_date' : IDL.Text,
    'question_asker' : IDL.Text,
    'question_logo' : IDL.Opt(IDL.Text),
    'question_description' : IDL.Text,
    'votes' : IDL.Nat32,
    'answers' : IDL.Vec(IDL.Tuple(IDL.Text, Answer)),
    'tags' : IDL.Vec(IDL.Text),
    'question_image' : IDL.Opt(IDL.Text),
    'question_id' : IDL.Text,
  });
  const SuccessResult_3 = IDL.Record({ 'question_list' : IDL.Vec(Question) });
  const Response_7 = IDL.Variant({ 'Success' : SuccessResult_3 });
  const SuccessResult_4 = IDL.Record({ 'question' : Question });
  const Response_8 = IDL.Variant({ 'Success' : SuccessResult_4 });
  const SuccessResult_5 = IDL.Record({ 'votes' : IDL.Nat32 });
  const Response_9 = IDL.Variant({ 'Success' : SuccessResult_5 });
  const Args_7 = IDL.Record({ 'owner' : IDL.Principal });
  const QuesAns = IDL.Record({
    'answers' : IDL.Vec(Answer),
    'questions' : IDL.Vec(Question),
    'watch_list' : IDL.Vec(IDL.Text),
  });
  const Profile = IDL.Record({
    'tvl' : IDL.Nat32,
    'tickets' : IDL.Nat32,
    'username' : IDL.Text,
    'owner' : IDL.Text,
    'qa_mod' : QuesAns,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'holders' : IDL.Nat32,
    'holding' : IDL.Nat32,
  });
  const SuccessResult_6 = IDL.Record({ 'profile' : Profile });
  const Response_10 = IDL.Variant({ 'Success' : SuccessResult_6 });
  const Args_8 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_7 = IDL.Record({ 'principal' : IDL.Principal });
  const Response_11 = IDL.Variant({ 'Success' : SuccessResult_7 });
  const Response_12 = IDL.Variant({
    'Success' : IDL.Null,
    'PrincipalInvalid' : IDL.Null,
  });
  const Args_9 = IDL.Record({
    'username' : IDL.Text,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'description' : IDL.Text,
  });
  const Response_13 = IDL.Variant({
    'Success' : IDL.Null,
    'PrincipalInvalid' : IDL.Null,
  });
  const Args_10 = IDL.Record({ 'question_id' : IDL.Text });
  const Response_14 = IDL.Variant({
    'Success' : IDL.Null,
    'UpVoteInvalid' : IDL.Null,
  });
  const Args_11 = IDL.Record({ 'owner' : IDL.Principal, 'name' : IDL.Text });
  const Response_15 = IDL.Variant({
    'TvlInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_12 = IDL.Record({
    'owner' : IDL.Principal,
    'description' : IDL.Text,
  });
  const Response_16 = IDL.Variant({
    'DescriptionInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_13 = IDL.Record({
    'owner' : IDL.Principal,
    'holders' : IDL.Nat32,
  });
  const Response_17 = IDL.Variant({
    'Success' : IDL.Null,
    'HoldersInvalid' : IDL.Null,
  });
  const Args_14 = IDL.Record({
    'owner' : IDL.Principal,
    'holding' : IDL.Nat32,
  });
  const Args_15 = IDL.Record({
    'tickets' : IDL.Nat32,
    'owner' : IDL.Principal,
  });
  const Args_16 = IDL.Record({ 'tvl' : IDL.Nat32, 'owner' : IDL.Principal });
  const Args_17 = IDL.Record({
    'username' : IDL.Text,
    'owner' : IDL.Principal,
  });
  const Args_18 = IDL.Record({
    'num_of_page' : IDL.Opt(IDL.Nat64),
    'page' : IDL.Nat64,
  });
  const SuccessResult_8 = IDL.Record({
    'question_list' : IDL.Tuple(IDL.Int32, IDL.Vec(Question)),
  });
  const Response_18 = IDL.Variant({ 'Success' : SuccessResult_8 });
  return IDL.Service({
    'add_new_answer' : IDL.Func([Args], [Response], []),
    'add_new_comment' : IDL.Func([Args_1], [Response], []),
    'add_new_question' : IDL.Func([Args_2], [Response_1], []),
    'add_profile_watch_list' : IDL.Func([Args_3], [Response_2], []),
    'down_vote' : IDL.Func([Args_4], [Response_3], []),
    'follow_operation' : IDL.Func([Args_5], [Response_3], []),
    'get_all_answers_list_by_question_id' : IDL.Func(
        [Args_4],
        [Response_4],
        ['query'],
      ),
    'get_all_comment_list' : IDL.Func([Args_6], [Response_5], ['query']),
    'get_all_question_id_list' : IDL.Func(
        [IDL.Record({})],
        [Response_6],
        ['query'],
      ),
    'get_all_question_list' : IDL.Func(
        [IDL.Record({})],
        [Response_7],
        ['query'],
      ),
    'get_question_by_id' : IDL.Func([Args_4], [Response_8], ['query']),
    'get_question_votes_by_id' : IDL.Func([Args_4], [Response_9], ['query']),
    'get_user_profile' : IDL.Func([Args_7], [Response_10], ['query']),
    'init_state' : IDL.Func([Args_8], [Response_11], ['query']),
    'set_user_principal' : IDL.Func([Args_8], [Response_12], []),
    'set_user_profile' : IDL.Func([Args_9], [Response_13], []),
    'un_follow_operation' : IDL.Func([Args_5], [Response_3], []),
    'up_vote' : IDL.Func([Args_10], [Response_14], []),
    'update_name' : IDL.Func([Args_11], [Response_15], []),
    'update_profile_description' : IDL.Func([Args_12], [Response_16], []),
    'update_user_holders' : IDL.Func([Args_13], [Response_17], []),
    'update_user_holding' : IDL.Func([Args_14], [Response_15], []),
    'update_user_tickets' : IDL.Func([Args_15], [Response_15], []),
    'update_user_tvl' : IDL.Func([Args_16], [Response_15], []),
    'update_username' : IDL.Func([Args_17], [Response_15], []),
    'view_by_page' : IDL.Func([Args_18], [Response_18], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
