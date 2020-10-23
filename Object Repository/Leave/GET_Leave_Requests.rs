<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Leave_Requests</name>
   <tag></tag>
   <elementGuidId>87c3fcfe-40ca-4d02-9abf-67604a85c91c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/api/v1/leave/search?fromDate=2020-10-27&amp;toDate=2020-10-28&amp;scheduled=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>329920c4-5d85-4284-ac7b-0b2e5771180b</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>df9e1f47-b87a-4129-a97c-0a7af6c4f3ab</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'data[0].employeeName', &quot;Zahra test test&quot;)
WS.verifyElementPropertyValue(response, 'data[0].employeeId', &quot;2&quot;)
WS.verifyElementPropertyValue(response, 'data[0].id', &quot;18&quot;)
WS.verifyElementPropertyValue(response, 'data[0].fromDate', &quot;2020-10-27&quot;)
WS.verifyElementPropertyValue(response, 'data[0].toDate', &quot;2020-10-28&quot;)
WS.verifyElementPropertyValue(response, 'data[0].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[0].leaveBalance', &quot;-5.00&quot;)
WS.verifyElementPropertyValue(response, 'data[0].numberOfDays', &quot;2.00&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[0].date', &quot;2020-10-28&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[0].status', &quot;SCHEDULED&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[0].duration', &quot;8.00&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[1].date', &quot;2020-10-27&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[1].status', &quot;SCHEDULED&quot;)
WS.verifyElementPropertyValue(response, 'data[0].days[1].duration', &quot;8.00&quot;)


WS.verifyElementPropertyValue(response, 'data[1].employeeName', &quot;Zahra01 test test&quot;)
WS.verifyElementPropertyValue(response, 'data[1].employeeId', &quot;14&quot;)
WS.verifyElementPropertyValue(response, 'data[1].id', &quot;19&quot;)
WS.verifyElementPropertyValue(response, 'data[1].fromDate', &quot;2020-10-28&quot;)
WS.verifyElementPropertyValue(response, 'data[1].toDate', &quot;2020-10-28&quot;)
WS.verifyElementPropertyValue(response, 'data[1].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[1].leaveBalance', &quot;-7.00&quot;)
WS.verifyElementPropertyValue(response, 'data[1].numberOfDays', &quot;1.00&quot;)
WS.verifyElementPropertyValue(response, 'data[1].days[0].date', &quot;2020-10-28&quot;)
WS.verifyElementPropertyValue(response, 'data[1].days[0].status', &quot;SCHEDULED&quot;)
WS.verifyElementPropertyValue(response, 'data[1].days[0].duration', &quot;8.00&quot;)

WS.verifyElementPropertyValue(response, 'data[2].employeeName', &quot;Zahra01 test test&quot;)
WS.verifyElementPropertyValue(response, 'data[2].employeeId', &quot;14&quot;)
WS.verifyElementPropertyValue(response, 'data[2].id', &quot;15&quot;)
WS.verifyElementPropertyValue(response, 'data[2].fromDate', &quot;2020-10-27&quot;)
WS.verifyElementPropertyValue(response, 'data[2].toDate', &quot;2020-10-27&quot;)
WS.verifyElementPropertyValue(response, 'data[2].type', &quot;Cuti&quot;)
WS.verifyElementPropertyValue(response, 'data[2].leaveBalance', &quot;-7.00&quot;)
WS.verifyElementPropertyValue(response, 'data[2].numberOfDays', &quot;1.00&quot;)
WS.verifyElementPropertyValue(response, 'data[2].days[0].date', &quot;2020-10-27&quot;)
WS.verifyElementPropertyValue(response, 'data[2].days[0].status', &quot;SCHEDULED&quot;)
WS.verifyElementPropertyValue(response, 'data[2].days[0].duration', &quot;8.00&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
