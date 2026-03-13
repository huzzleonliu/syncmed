-- 将历史数据状态统一到 request sent / finished
update patients
set report_status = case
    when report_status in ('completed', 'finished') then 'finished'
    else 'request sent'
end;

-- request sent 状态的完成时间必须为空
update patients
set completed_at = null
where report_status = 'request sent';

-- finished 状态如果无完成时间，则回填为请求时间（避免约束失败）
update patients
set completed_at = requested_at
where report_status = 'finished'
  and completed_at is null;

-- 添加状态与时间一致性约束
alter table patients
    add constraint chk_patients_report_status
    check (report_status in ('request sent', 'finished'));

alter table patients
    add constraint chk_patients_status_time_consistency
    check (
        (report_status = 'request sent' and completed_at is null)
        or
        (report_status = 'finished' and completed_at is not null)
    );
