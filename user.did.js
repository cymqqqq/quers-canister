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
    'question_description' : IDL.Opt(IDL.Text),
    'reference_link' : IDL.Opt(IDL.Text),
    'lang' : IDL.Text,
    'tags' : IDL.Opt(IDL.Vec(IDL.Text)),
    'reference_title' : IDL.Opt(IDL.Text),
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
  const Args_4 = IDL.Record({ 'to' : IDL.Principal, 'owner' : IDL.Principal });
  const FollowStatus = IDL.Variant({
    'UN_FOLLOWED' : IDL.Null,
    'FOLLOWING' : IDL.Null,
    'FOLLOWED' : IDL.Null,
  });
  const SuccessResult = IDL.Record({ 'follow_status' : FollowStatus });
  const Response_3 = IDL.Variant({ 'Success' : SuccessResult });
  const Args_5 = IDL.Record({ 'question_id' : IDL.Text });
  const Response_4 = IDL.Variant({
    'DownVoteInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_6 = IDL.Record({
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
  const SuccessResult_1 = IDL.Record({ 'answer_list' : IDL.Vec(Answer) });
  const Response_5 = IDL.Variant({ 'Success' : SuccessResult_1 });
  const Args_7 = IDL.Record({
    'question_id' : IDL.Text,
    'answer_pid' : IDL.Principal,
  });
  const SuccessResult_2 = IDL.Record({ 'comment_list' : IDL.Vec(Comment) });
  const Response_6 = IDL.Variant({ 'Success' : SuccessResult_2 });
  const SuccessResult_3 = IDL.Record({
    'question_id_list' : IDL.Vec(IDL.Text),
  });
  const Response_7 = IDL.Variant({ 'Success' : SuccessResult_3 });
  const Question = IDL.Record({
    'question_title' : IDL.Text,
    'question_date' : IDL.Text,
    'question_asker' : IDL.Text,
    'question_description' : IDL.Text,
    'reference_link' : IDL.Text,
    'votes' : IDL.Nat32,
    'answers' : IDL.Vec(IDL.Tuple(IDL.Text, Answer)),
    'tags' : IDL.Vec(IDL.Text),
    'reference_title' : IDL.Text,
    'question_image' : IDL.Text,
    'question_id' : IDL.Text,
  });
  const SuccessResult_4 = IDL.Record({ 'question_list' : IDL.Vec(Question) });
  const Response_8 = IDL.Variant({ 'Success' : SuccessResult_4 });
  const Args_8 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_5 = IDL.Record({
    'question_answer_list' : IDL.Vec(Question),
  });
  const Response_9 = IDL.Variant({ 'Success' : SuccessResult_5 });
  const Args_9 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_6 = IDL.Record({ 'followers' : IDL.Nat64 });
  const Response_10 = IDL.Variant({ 'Success' : SuccessResult_6 });
  const Args_10 = IDL.Record({ 'owner' : IDL.Principal });
  const QuesAns = IDL.Record({
    'answers' : IDL.Vec(IDL.Text),
    'questions' : IDL.Vec(IDL.Text),
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
    'profile_image_url' : IDL.Text,
  });
  const SuccessResult_7 = IDL.Record({ 'followers_set' : IDL.Vec(Profile) });
  const Response_11 = IDL.Variant({ 'Success' : SuccessResult_7 });
  const SuccessResult_8 = IDL.Record({ 'followings' : IDL.Nat64 });
  const Response_12 = IDL.Variant({ 'Success' : SuccessResult_8 });
  const SuccessResult_9 = IDL.Record({ 'followings_set' : IDL.Vec(Profile) });
  const Response_13 = IDL.Variant({ 'Success' : SuccessResult_9 });
  const Args_11 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_10 = IDL.Record({ 'watch_list' : IDL.Vec(Question) });
  const Response_14 = IDL.Variant({ 'Success' : SuccessResult_10 });
  const SuccessResult_11 = IDL.Record({ 'question' : Question });
  const Response_15 = IDL.Variant({ 'Success' : SuccessResult_11 });
  const SuccessResult_12 = IDL.Record({ 'votes' : IDL.Nat32 });
  const Response_16 = IDL.Variant({ 'Success' : SuccessResult_12 });
  const Args_12 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_13 = IDL.Record({ 'profile' : Profile });
  const Response_17 = IDL.Variant({ 'Success' : SuccessResult_13 });
  const Args_13 = IDL.Record({ 'owner' : IDL.Principal });
  const SuccessResult_14 = IDL.Record({ 'principal' : IDL.Principal });
  const Response_18 = IDL.Variant({ 'Success' : SuccessResult_14 });
  const Args_14 = IDL.Record({
    'username' : IDL.Text,
    'owner' : IDL.Principal,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'profile_image_url' : IDL.Text,
  });
  const Response_19 = IDL.Variant({
    'Success' : IDL.Null,
    'PrincipalInvalid' : IDL.Null,
  });
  const Args_15 = IDL.Record({ 'question_id' : IDL.Text });
  const Response_20 = IDL.Variant({
    'Success' : IDL.Null,
    'UpVoteInvalid' : IDL.Null,
  });
  const Args_16 = IDL.Record({ 'owner' : IDL.Principal, 'name' : IDL.Text });
  const Response_21 = IDL.Variant({
    'TvlInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_17 = IDL.Record({
    'owner' : IDL.Principal,
    'description' : IDL.Text,
  });
  const Response_22 = IDL.Variant({
    'DescriptionInvalid' : IDL.Null,
    'Success' : IDL.Null,
  });
  const Args_18 = IDL.Record({
    'owner' : IDL.Principal,
    'holders' : IDL.Nat32,
  });
  const Response_23 = IDL.Variant({
    'Success' : IDL.Null,
    'HoldersInvalid' : IDL.Null,
  });
  const Args_19 = IDL.Record({
    'owner' : IDL.Principal,
    'holding' : IDL.Nat32,
  });
  const Args_20 = IDL.Record({
    'tickets' : IDL.Nat32,
    'owner' : IDL.Principal,
  });
  const Args_21 = IDL.Record({ 'tvl' : IDL.Nat32, 'owner' : IDL.Principal });
  const Args_22 = IDL.Record({
    'username' : IDL.Text,
    'owner' : IDL.Principal,
  });
  const Args_23 = IDL.Record({
    'num_of_page' : IDL.Opt(IDL.Nat64),
    'page' : IDL.Nat64,
  });
  const SuccessResult_15 = IDL.Record({
    'question_list' : IDL.Tuple(IDL.Int32, IDL.Vec(Question)),
  });
  const Response_24 = IDL.Variant({ 'Success' : SuccessResult_15 });
  return IDL.Service({
    'add_new_answer' : IDL.Func([Args], [Response], []),
    'add_new_comment' : IDL.Func([Args_1], [Response], []),
    'add_new_question' : IDL.Func([Args_2], [Response_1], []),
    'add_profile_watch_list' : IDL.Func([Args_3], [Response_2], []),
    'check_profile_follow_status' : IDL.Func([Args_4], [Response_3], ['query']),
    'down_vote' : IDL.Func([Args_5], [Response_4], []),
    'follow_operation' : IDL.Func([Args_6], [Response_4], []),
    'get_all_answers_list_by_question_id' : IDL.Func(
        [Args_5],
        [Response_5],
        ['query'],
      ),
    'get_all_comment_list' : IDL.Func([Args_7], [Response_6], ['query']),
    'get_all_question_id_list' : IDL.Func(
        [IDL.Record({})],
        [Response_7],
        ['query'],
      ),
    'get_all_question_list' : IDL.Func(
        [IDL.Record({})],
        [Response_8],
        ['query'],
      ),
    'get_profile_answer_question_list' : IDL.Func(
        [Args_8],
        [Response_9],
        ['query'],
      ),
    'get_profile_followers_count' : IDL.Func(
        [Args_9],
        [Response_10],
        ['query'],
      ),
    'get_profile_followers_set' : IDL.Func([Args_10], [Response_11], ['query']),
    'get_profile_followings_count' : IDL.Func(
        [Args_10],
        [Response_12],
        ['query'],
      ),
    'get_profile_followings_set' : IDL.Func(
        [Args_10],
        [Response_13],
        ['query'],
      ),
    'get_profile_question_list' : IDL.Func([Args_10], [Response_8], ['query']),
    'get_profile_watch_list' : IDL.Func([Args_11], [Response_14], ['query']),
    'get_question_by_id' : IDL.Func([Args_5], [Response_15], ['query']),
    'get_question_votes_by_id' : IDL.Func([Args_5], [Response_16], ['query']),
    'get_user_profile' : IDL.Func([Args_12], [Response_17], ['query']),
    'init_state' : IDL.Func([Args_13], [Response_18], ['query']),
    'set_user_profile' : IDL.Func([Args_14], [Response_19], []),
    'un_follow_operation' : IDL.Func([Args_6], [Response_4], []),
    'up_vote' : IDL.Func([Args_15], [Response_20], []),
    'update_name' : IDL.Func([Args_16], [Response_21], []),
    'update_profile_description' : IDL.Func([Args_17], [Response_22], []),
    'update_user_holders' : IDL.Func([Args_18], [Response_23], []),
    'update_user_holding' : IDL.Func([Args_19], [Response_21], []),
    'update_user_tickets' : IDL.Func([Args_20], [Response_21], []),
    'update_user_tvl' : IDL.Func([Args_21], [Response_21], []),
    'update_username' : IDL.Func([Args_22], [Response_21], []),
    'view_by_page' : IDL.Func([Args_23], [Response_24], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
